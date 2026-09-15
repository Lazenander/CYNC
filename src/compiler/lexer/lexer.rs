use crate::compiler::lexer::tokens::{LexicalError, Position, Token, TokenGut};
use logos::{Logos, SpannedIter};
use std::path::PathBuf;

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

#[derive(Debug, Clone)]
pub struct LexResult {
    pub tokens: Vec<Token>,
    pub errors: Vec<(LexicalError, Position)>,
}

impl LexResult {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    pub fn add_error(&mut self, error: LexicalError, position: Position) {
        self.errors.push((error, position));
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn is_success(&self) -> bool {
        self.errors.is_empty()
    }
}

struct Lexer<'input> {
    token_stream: SpannedIter<'input, TokenGut>,
    eof_emitted: bool,
    input_len: usize,
    input: &'input str,
    path: std::path::PathBuf,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str, path: std::path::PathBuf) -> Self {
        let input_len = input.len();
        Self {
            token_stream: TokenGut::lexer(input).spanned(),
            eof_emitted: false,
            input_len,
            input,
            path,
        }
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Result<Token, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((token_result, span)) = self.token_stream.next() {
            match token_result {
                Ok(token) => {
                    let position =
                        Position::from_span(self.path.clone(), (span.start, span.end), self.input);
                    Some(Ok(Token::new(token, position)))
                }
                Err(err) => Some(Err(err)),
            }
        } else if !self.eof_emitted {
            self.eof_emitted = true;
            let position = Position::new(self.path.clone(), self.input_len, 0, 0, self.input_len);
            Some(Ok(Token::new(TokenGut::EOF, position)))
        } else {
            None
        }
    }
}

/// Lex a string input into tokens and collect all errors
pub fn lex(input: &str, path: PathBuf) -> LexResult {
    let mut lexer = Lexer::new(input, path.clone());
    let mut result = LexResult::new();
    let mut current_pos = 0;

    while let Some(token_result) = lexer.next() {
        match token_result {
            Ok(token) => {
                current_pos = token.position.end;
                result.add_token(token);
            }
            Err(error) => {
                // For lexical errors, use the current position
                let position = Position::new(path.clone(), current_pos, current_pos + 1, 0, 0);
                result.add_error(error, position);
                // Skip one character and continue
                current_pos += 1;
            }
        }
    }

    result
}
