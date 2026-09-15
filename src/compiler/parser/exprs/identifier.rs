use crate::compiler::lexer::tokens::{Operator, Position, TokenGut};
use crate::compiler::parser::error::ParseError;
use crate::compiler::parser::exprs;
use crate::compiler::parser::exprs::{Expr, ExprStructure, EXPR_SYNC_SET};
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};

#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: String,
}

impl ParsedBox<Identifier> {
    pub fn get_name(&self) -> String {
        self.value().name.clone()
    }
}

impl Default for Identifier {
    fn default() -> Self {
        Identifier {
            name: "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RoutedIdentifier {
    pub route: Vec<ParsedBox<Identifier>>,
    pub this_id: ParsedBox<Identifier>,
}

impl RoutedIdentifier {
    pub fn de_identifier(&self) -> (Vec<String>, String) {
        (
            self.route.iter().map(|item| item.get_name()).collect(),
            self.this_id.get_name(),
        )
    }

    pub fn to_vec_string(&self) -> Vec<String> {
        let (module_route, name) = self.de_identifier();
        module_route.into_iter().chain(vec![name]).collect()
    }
}

impl ParsedBox<RoutedIdentifier> {
    pub fn gen_pb_identifier(&self) -> Option<ParsedBox<Identifier>> {
        if !self.value().route.is_empty() {
            return None;
        }
        Some(self.value().this_id.clone())
    }
}

impl Parsed<RoutedIdentifier> {
    pub fn gen_pb_identifier(&self) -> Option<Parsed<ParsedBox<Identifier>>> {
        if !self.value.route.is_empty() {
            return None;
        }
        Some(Parsed::lift_parsed(self.clone(), |v| v.this_id.clone()))
    }
}

impl Parser {
    pub fn default_routed_identifier(&self) -> RoutedIdentifier {
        RoutedIdentifier {
            route: vec![],
            this_id: self.new_parsed_box(Default::default()),
        }
    }

    pub fn try_parse_routes(&mut self) -> Option<Parsed<Vec<ParsedBox<Identifier>>>> {
        self.safe_try(|s_self| {
            Some(s_self.parse_star(|c_self| {
                let p_id = c_self.try_parse_identifier()?;
                let p_dc = c_self.try_parse_operator(Operator::DoubleColon)?;
                Some(Parsed::merge_parsed_ignore_right(p_id, p_dc))
            }))
        })
    }

    pub fn parse_routed_identifier(&mut self) -> Parsed<RoutedIdentifier> {
        self.force_parse_with_default(
            self.default_routed_identifier(),
            Parser::try_parse_routed_identifier,
            ParseError::expected_identifier_not_found.clone(),
            EXPR_SYNC_SET.into(),
        )
    }

    pub fn try_parse_routed_identifier(&mut self) -> Option<Parsed<RoutedIdentifier>> {
        self.safe_try(|s_self| {
            let p_route = s_self.try_parse_routes()?;
            let p_id = Parsed::box_parsed(s_self.parse_identifier());
            Some(Parsed::merge_parsed(p_route, p_id, |route, this_id| {
                RoutedIdentifier { route, this_id }
            }))
        })
    }

    pub fn parse_identifier_expr(&mut self) -> Parsed<Expr> {
        self.force_parse_with_default(
            self.default_expr(),
            Parser::try_parse_identifier_expr,
            ParseError::expected_identifier_not_found.clone(),
            EXPR_SYNC_SET.into(),
        )
    }

    pub fn try_parse_identifier_expr(&mut self) -> Option<Parsed<Expr>> {
        self.safe_try(|s_self| {
            Some(Parsed::lift_parsed(
                s_self.try_parse_routed_identifier()?,
                |identifier| {
                    Expr::new(ExprStructure::Identifier(s_self.new_parsed_box(identifier)))
                },
            ))
        })
    }

    pub fn parse_identifier(&mut self) -> Parsed<Identifier> {
        self.force_parse_with_default(
            Default::default(),
            Parser::try_parse_identifier,
            ParseError::expected_identifier_not_found.clone(),
            EXPR_SYNC_SET.into(),
        )
    }

    pub fn try_parse_identifier(&mut self) -> Option<Parsed<Identifier>> {
        self.safe_try(|s_self| {
            let token = s_self.current_token()?;
            let TokenGut::Identifier(name) = token.t_gut else {
                return None;
            };
            s_self.next();
            Some(s_self.new_value(exprs::identifier::Identifier { name }))
        })
    }
}
