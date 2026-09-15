use crate::compiler::lexer::tokens::{Position, Token, TokenGut};
use crate::compiler::parser::error::{ParseError, ParseErrors};
use crate::compiler::parser::program::program::Program;
use crate::utility::vpe::VPE;
use std::cmp::{max, min};
use std::iter;
use std::ops::{Add, Deref};
use std::path::PathBuf;

#[derive(Default, Debug, Clone)]
pub struct ParsedPosition {
    pub path: PathBuf,
    pub start: usize,
    pub end: usize,
    pub start_line: usize,
    pub start_column: usize,
    pub end_line: usize,
    pub end_column: usize,
}

impl Add for ParsedPosition {
    type Output = ParsedPosition;

    fn add(self, other: ParsedPosition) -> ParsedPosition {
        let start_pos1 = (self.start_line, self.start_column);
        let start_pos2 = (other.start_line, other.start_column);
        let start_pos: (usize, usize);
        if start_pos1.0 == start_pos2.0 {
            start_pos = (start_pos1.0, min(start_pos1.1, start_pos2.1));
        } else if start_pos1.0 < start_pos2.0 {
            start_pos = start_pos1;
        } else {
            start_pos = start_pos2;
        }

        let end_pos1 = (self.end_line, self.end_column);
        let end_pos2 = (other.end_line, other.end_column);
        let end_pos: (usize, usize);
        if end_pos1.0 == end_pos2.0 {
            end_pos = (end_pos1.0, max(end_pos1.1, end_pos2.1));
        } else if end_pos1.0 > end_pos2.0 {
            end_pos = end_pos1;
        } else {
            end_pos = end_pos2;
        }

        ParsedPosition {
            path: self.path,
            start: min(self.start, other.start),
            end: max(self.end, other.end),
            start_line: start_pos.0,
            start_column: start_pos.1,
            end_line: end_pos.0,
            end_column: end_pos.1,
        }
    }
}

impl ParsedPosition {
    pub fn new(pos: Position) -> ParsedPosition {
        ParsedPosition {
            path: pos.path,
            start: pos.start,
            end: pos.end,
            start_line: pos.line,
            start_column: pos.column,
            end_line: pos.line,
            end_column: pos.column,
        }
    }

    pub fn print_path_line_column_span(&self) -> String {
        format!(
            "{}, Line {}, Column {} to Line {}, Column {}",
            self.path.display(),
            self.start_line,
            self.start_column,
            self.end_line,
            self.end_column
        )
    }
}

#[derive(Debug, Clone)]
pub struct ParsedBox<T> {
    pub value: Box<T>,
    pub position: ParsedPosition,
}

impl<T> ParsedBox<T> {
    pub fn new_with_pos(value: T, position: Position) -> ParsedBox<T> {
        ParsedBox {
            value: Box::new(value),
            position: ParsedPosition::new(position),
        }
    }

    pub fn new(value: T, position: ParsedPosition) -> ParsedBox<T> {
        ParsedBox {
            value: Box::new(value),
            position,
        }
    }

    pub fn owned_value(self) -> T {
        *self.value
    }

    pub fn value(&self) -> &T {
        &(*self.value)
    }

    pub fn deref(self) -> (T, ParsedPosition) {
        (*(self.value), self.position)
    }

    pub fn ref_map<F, T2>(&self, f: F) -> ParsedBox<T2>
    where
        F: FnOnce(&T) -> T2,
    {
        ParsedBox {
            value: Box::new(f(self.value())),
            position: self.position.clone(),
        }
    }

    pub fn map<F, T2>(self, f: F) -> ParsedBox<T2>
    where
        F: FnOnce(T) -> T2,
    {
        ParsedBox {
            value: Box::new(f(*self.value)),
            position: self.position,
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

#[derive(Debug, Clone)]
pub struct Parsed<T> {
    pub value: T,
    pub position: ParsedPosition,
    pub error: ParseErrors,
}

impl<V: Sized> VPE<V> for Parsed<V> {
    type P = ParsedPosition;
    type E = ParseErrors;

    fn get_value(&self) -> &V {
        &self.value
    }
    fn get_position(&self) -> &Self::P {
        &self.position
    }
    fn get_errors(&self) -> &Self::E {
        &self.error
    }

    fn transfer_vpe_ownership(self) -> (V, Self::P, Self::E) {
        (self.value, self.position, self.error)
    }

    fn set_value(&mut self, value: V) {
        self.value = value
    }
    fn set_position(&mut self, position: Self::P) {
        self.position = position
    }
    fn set_error(&mut self, error: Self::E) {
        self.error = error
    }

    fn new(value: V, position: Self::P, error: Self::E) -> Self {
        Self {
            value,
            position,
            error,
        }
    }
}

impl<T> Parsed<T> {
    pub fn lift_parsed<F, T1>(parsed: Parsed<T>, lift: F) -> Parsed<T1>
    where
        F: FnOnce(T) -> T1,
    {
        Self::lift_same_pe(parsed, lift)
    }

    pub fn box_parsed(parsed: Parsed<T>) -> Parsed<ParsedBox<T>> {
        let position = parsed.position.clone();
        Self::lift_parsed(parsed, |v| ParsedBox::new(v, position))
    }

    pub fn merge_parsed_op<T1, F>(
        parsed_lhs: Parsed<T>,
        parsed_op: Parsed<T1>,
        parsed_rhs: Parsed<T>,
        merge_value: F,
    ) -> Parsed<T>
    where
        F: FnOnce(T, T1, T) -> T,
    {
        Parsed::merge_parsed_3(parsed_lhs, parsed_op, parsed_rhs, merge_value)
    }

    pub fn merge_parsed<T1, T2, F>(
        parsed1: Parsed<T1>,
        parsed2: Parsed<T2>,
        merge_value: F,
    ) -> Parsed<T>
    where
        F: FnOnce(T1, T2) -> T,
    {
        Self::merge_same_pe(parsed1, parsed2, merge_value)
    }

    pub fn merge_parsed_3<T1, T2, T3, F>(
        parsed1: Parsed<T1>,
        parsed2: Parsed<T2>,
        parsed3: Parsed<T3>,
        merge_value: F,
    ) -> Parsed<T>
    where
        F: FnOnce(T1, T2, T3) -> T,
    {
        Self::merge_3_same_pe(parsed1, parsed2, parsed3, merge_value)
    }

    pub fn merge_parsed_append(parsed1: Parsed<Vec<T>>, parsed2: Parsed<T>) -> Parsed<Vec<T>> {
        Parsed::<Vec<T>>::merge_parsed(parsed1, parsed2, |vec, new_value| {
            vec.into_iter().chain(iter::once(new_value)).collect()
        })
    }

    pub fn merge_parsed_ignore_left<T1>(parsed1: Parsed<T1>, parsed2: Parsed<T>) -> Parsed<T> {
        Parsed::merge_parsed(parsed1, parsed2, |_, value| value)
    }

    pub fn merge_parsed_ignore_right<T1>(parsed1: Parsed<T>, parsed2: Parsed<T1>) -> Parsed<T> {
        Parsed::merge_parsed(parsed1, parsed2, |value, _| value)
    }

    pub fn merge_parsed_ignore_left_right<T1, T2>(
        parsed_left: Parsed<T1>,
        parsed_middle: Parsed<T>,
        parsed_right: Parsed<T2>,
    ) -> Parsed<T> {
        Parsed::merge_parsed_3(parsed_left, parsed_middle, parsed_right, |_, m, _| m)
    }

    pub fn merge_parsed_ignore_middle<T1, T2, T3, F>(
        parsed_left: Parsed<T1>,
        parsed_middle: Parsed<T2>,
        parsed_right: Parsed<T3>,
        merge_value: F,
    ) -> Parsed<T>
    where
        F: FnOnce(T1, T3) -> T,
    {
        Parsed::merge_parsed_3(parsed_left, parsed_middle, parsed_right, |l, _, r| {
            merge_value(l, r)
        })
    }

    pub fn new_value(value: T, position: Position) -> Parsed<T> {
        Self::new(
            value,
            ParsedPosition::new(position),
            ParseErrors::from(Vec::new()),
        )
    }

    pub fn new_error(error: ParseError, position: Position) -> Parsed<()> {
        Parsed::new(
            (),
            ParsedPosition::new(position),
            ParseErrors::from(vec![error]),
        )
    }

    pub fn new_error_with_default(error: ParseError, default: T, position: Position) -> Parsed<T> {
        Parsed::new(
            default,
            ParsedPosition::new(position),
            ParseErrors::from(vec![error]),
        )
    }

    pub fn push_option(
        optioned_parsed: Option<Parsed<T>>,
        none_position: Position,
    ) -> Parsed<Option<T>> {
        match optioned_parsed {
            Some(parsed) => Parsed::new(Some(parsed.value), parsed.position, parsed.error),
            None => Parsed::new(
                None,
                ParsedPosition::new(none_position),
                ParseErrors::from(Vec::new()),
            ),
        }
    }

    pub fn push_vec(vec_parsed: Vec<Parsed<T>>, vec_position: Position) -> Parsed<Vec<T>> {
        let mut p_list: Parsed<Vec<T>> = Parsed::new_value(vec![], vec_position);
        for parsed in vec_parsed {
            p_list = Parsed::merge_parsed_append(p_list, parsed);
        }
        p_list
    }

    pub fn unwrap_option(optioned_parsed: Option<Parsed<T>>, error_parsed: Parsed<T>) -> Parsed<T> {
        optioned_parsed.unwrap_or(error_parsed)
    }
}

impl Parser {
    pub fn new_parsed_box<T: Clone>(&self, value: T) -> ParsedBox<T> {
        ParsedBox::new(value, ParsedPosition::new(self.current_position()))
    }

    pub fn new_value<T: Clone>(&self, value: T) -> Parsed<T> {
        Parsed::new_value(value, self.current_position())
    }

    pub fn new_error(&self, error: ParseError) -> Parsed<()> {
        Parsed::<()>::new_error(error, self.current_position())
    }

    pub fn new_error_with_default<T: Clone>(&self, error: ParseError, default: T) -> Parsed<T> {
        Parsed::new_error_with_default(error, default, self.current_position())
    }

    pub fn push_option<T: Clone>(&self, optioned_parsed: Option<Parsed<T>>) -> Parsed<Option<T>> {
        Parsed::push_option(optioned_parsed, self.current_position())
    }

    pub fn push_vec<T: Clone>(&self, vec_parsed: Vec<Parsed<T>>) -> Parsed<Vec<T>> {
        Parsed::push_vec(vec_parsed, self.current_position())
    }

    pub fn unwrap_option<T: Clone>(
        &self,
        optioned_parsed: Option<Parsed<T>>,
        error_parsed: Parsed<T>,
    ) -> Parsed<T> {
        Parsed::unwrap_option(optioned_parsed, error_parsed)
    }

    pub fn safe_try<T, F>(&mut self, try_parse: F) -> Option<T>
    where
        F: FnOnce(&mut Self) -> Option<T>,
    {
        let rec_index = self.index.clone();
        let res = try_parse(self);
        match res {
            Some(res) => Some(res),
            None => {
                self.index = rec_index;
                None
            }
        }
    }

    pub fn peek_try<T, F>(&mut self, try_parse: F) -> Option<T>
    where
        F: FnOnce(&mut Self) -> Option<T>,
    {
        let rec_index = self.index.clone();
        let res = try_parse(self);
        self.index = rec_index;
        res
    }

    pub fn passable_safe_try<T, F1, F2>(&mut self, try_parse: F1, pass: F2) -> Option<T>
    where
        T: Clone,
        F1: FnOnce(&mut Self) -> Option<T>,
        F2: FnOnce(T) -> bool,
    {
        let rec_index = self.index.clone();
        let res = try_parse(self);
        match res {
            Some(res) => {
                if pass(res.clone()) {
                    Some(res)
                } else {
                    self.index = rec_index;
                    None
                }
            }
            None => {
                self.index = rec_index;
                None
            }
        }
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        let tokens: Vec<Token> = tokens
            .into_iter()
            .filter(|t| !matches!(t.t_gut, TokenGut::Comment(_)))
            .collect();
        Self { tokens, index: 0 }
    }

    pub fn next(&mut self) -> () {
        self.index += 1;
    }

    pub fn prior_token(&self) -> Option<Token> {
        if self.index == 0 {
            None
        } else {
            self.tokens.get(self.index - 1).cloned()
        }
    }

    pub fn current_position(&self) -> Position {
        self.offset_token(0).unwrap_or(self.first_token()).position
    }

    pub fn current_token(&self) -> Option<Token> {
        self.offset_token(0)
    }

    pub fn next_token(&self) -> Option<Token> {
        self.offset_token(1)
    }

    pub fn offset_token(&self, offset: usize) -> Option<Token> {
        self.tokens.get(self.index + offset).cloned()
    }

    pub fn first_token(&self) -> Token {
        self.tokens
            .first()
            .cloned()
            .unwrap_or_else(|| unreachable!())
    }

    pub fn final_token(&self) -> Token {
        self.tokens
            .last()
            .cloned()
            .unwrap_or_else(|| unreachable!())
    }

    pub fn parse_cync(&mut self) -> Parsed<ParsedBox<Program>> {
        let p_program = Parsed::box_parsed(self.parse_program());
        if self.index != self.tokens.len() {
            Parsed::merge_parsed_ignore_right(
                p_program,
                self.new_error(ParseError::finish_without_reaching_eof()),
            )
        } else {
            p_program
        }
    }
}

impl Parser {
    pub fn force_parse_with_default<T, F1, F2>(
        &mut self,
        default_value: T,
        try_parse: F1,
        error_gen: F2,
        sync_set: Vec<TokenGut>,
    ) -> Parsed<T>
    where
        T: Clone,
        F1: FnOnce(&mut Self) -> Option<Parsed<T>>,
        F2: FnOnce(Token) -> ParseError,
    {
        let option_parsed = try_parse(self);
        match option_parsed {
            Some(parsed) => parsed,
            None => {
                if sync_set.is_empty() {
                    self.next();
                } else {
                    loop {
                        match self.current_token() {
                            Some(tok) => {
                                let gut = tok.t_gut.clone();
                                let is_identifier = matches!(gut, TokenGut::Identifier(_));
                                let contains_identifier_wildcard =
                                    sync_set.contains(&TokenGut::Identifier("".into()));
                                if sync_set.contains(&gut)
                                    || (contains_identifier_wildcard && is_identifier)
                                    || matches!(gut, TokenGut::EOF)
                                {
                                    break;
                                }
                                self.next();
                            }
                            None => break,
                        }
                    }
                }
                let at_eof = matches!(
                    self.current_token()
                        .unwrap_or_else(|| self.final_token())
                        .t_gut,
                    TokenGut::EOF
                );
                let err = if at_eof {
                    ParseError::expected_eof_not_found(self.current_position())
                } else {
                    error_gen(self.current_token().unwrap_or_else(|| self.final_token()))
                };
                Parsed::new(
                    default_value,
                    ParsedPosition::new(self.current_position()),
                    ParseErrors::from(vec![err]),
                )
            }
        }
    }

    pub fn parse_star<T: Clone, F>(&mut self, try_parse: F) -> Parsed<Vec<ParsedBox<T>>>
    where
        F: Fn(&mut Self) -> Option<Parsed<T>>,
    {
        let mut p_list = Parsed::new(
            Vec::new(),
            ParsedPosition::new(self.current_position()),
            ParseErrors::from(Vec::new()),
        );
        let safe_try_parse = |c_self: &mut Parser| c_self.safe_try(|s_self| try_parse(s_self));
        while let Some(next) = safe_try_parse(self) {
            p_list = Parsed::merge_parsed_append(p_list, Parsed::box_parsed(next));
        }
        p_list
    }

    pub fn parse_addition<T1: Clone, T2, F1, F2>(
        &mut self,
        parse_content: F1,
        try_parse_separator: F2,
    ) -> Parsed<Vec<ParsedBox<T1>>>
    where
        F1: Fn(&mut Self) -> Parsed<T1>,
        F2: Fn(&mut Self) -> Option<Parsed<T2>>,
    {
        let mut p_list = Parsed::new(
            Vec::new(),
            ParsedPosition::new(self.current_position()),
            ParseErrors::from(Vec::new()),
        );
        p_list = Parsed::merge_parsed_append(p_list, Parsed::box_parsed(parse_content(self)));
        while let Some(p_sep) = try_parse_separator(self) {
            p_list = Parsed::merge_parsed_append(
                p_list,
                Parsed::box_parsed(Parsed::merge_parsed_ignore_left(p_sep, parse_content(self))),
            );
        }
        p_list
    }

    pub fn try_parse_addition<T1: Clone, T2, F1, F2, F3>(
        &mut self,
        try_parse_content: F1,
        parse_content: F2,
        try_parse_separator: F3,
    ) -> Option<Parsed<Vec<ParsedBox<T1>>>>
    where
        F1: FnOnce(&mut Self) -> Option<Parsed<T1>>,
        F2: Fn(&mut Self) -> Parsed<T1>,
        F3: Fn(&mut Self) -> Option<Parsed<T2>>,
    {
        self.safe_try(|s_self| {
            let mut p_list = Parsed::new(
                Vec::new(),
                ParsedPosition::new(s_self.current_position()),
                ParseErrors::from(Vec::new()),
            );
            p_list =
                Parsed::merge_parsed_append(p_list, Parsed::box_parsed(try_parse_content(s_self)?));
            while let Some(p_sep) = try_parse_separator(s_self) {
                p_list = Parsed::merge_parsed_append(
                    p_list,
                    Parsed::box_parsed(Parsed::merge_parsed_ignore_right(
                        parse_content(s_self),
                        p_sep,
                    )),
                );
            }
            Some(p_list)
        })
    }

    pub fn chain_try_parses<T>(
        &mut self,
        try_parses: Vec<fn(&mut Parser) -> Option<Parsed<T>>>,
    ) -> Option<Parsed<T>> {
        self.safe_try(|s_self| {
            for try_parse in try_parses {
                let o_p = try_parse(s_self);
                if let Some(p) = o_p {
                    return Some(p);
                }
            }
            return None;
        })
    }

    pub fn chain_try_parses_with_default_parse<T>(
        &mut self,
        try_parses: Vec<fn(&mut Parser) -> Option<Parsed<T>>>,
        default: fn(&mut Parser) -> Parsed<T>,
    ) -> Parsed<T> {
        self.chain_try_parses(try_parses).unwrap_or(default(self))
    }
}
