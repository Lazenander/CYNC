use crate::compiler::lexer::to_string;
use crate::compiler::lexer::tokens::{Keyword, Paren, Position, Token, TokenGut};
use crate::utility::common::vec_add::AddableVec;
use std::fmt;
use std::ops::Add;

#[derive(Debug, Clone)]
pub enum ErrorKind {
    EXPECTED_TOKEN_NOT_FOUND,
    EXPECTED_PAREN_NOT_FOUND,
    EXPECTED_EXPRESSION_NOT_FOUND,
    EXPECTED_IDENTIFIER_NOT_FOUND,
    EXPECTED_LITERAL_NOT_FOUND,
    EXPECTED_SEMICOLON_NOT_FOUND,
    EXPECTED_COMMA_NOT_FOUND,
    EXPECTED_PATTERN_NOT_FOUND,
    EXPECTED_TYPE_PATTERN_NOT_FOUND,
    EXPECTED_TYPE_NOT_FOUND,
    EXPECTED_STRUCT_TYPE_ELEMENT_NOT_FOUND,
    EXPECTED_STRUCT_PATTERN_ELEMENT_NOT_FOUND,
    EXPECTED_STRUCT_ELEMENT_NOT_FOUND,
    EXPECTED_ENUM_CASE_NOT_FOUND,
    EXPECTED_STRUCT_TYPE_PATTERN_ELEMENT_NOT_FOUND,
    EXPECTED_FUNCTION_ARG_NOT_FOUND,
    EXPECTED_FUNCTION_BLOCK_NOT_FOUND,
    EXPECTED_TEMPLATE_DEF_ELEMENT_NOT_FOUND,
    EXPECTED_DEFINITION_STATEMENT_NOT_FOUND,
    EXPECTED_IN_BIND_STATEMENT_NOT_FOUND,
    EXPECTED_STATEMENT_NOT_FOUND,
    EXPECTED_STATEMENT_BLOCK_NOT_FOUND,
    EXPECTED_METHOD_STATEMENT_IN_IMPLEMENTATION_NOT_FOUND,
    EXPECTED_EOF_NOT_FOUND,
    END_WITHOUT_REACHING_EOF,
}

#[derive(Debug, Clone)]
pub struct ParseError {
    pub kind: ErrorKind,
    pub position: Option<Position>,
    pub expected: Option<TokenGut>,
    pub found: Option<TokenGut>,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            ErrorKind::EXPECTED_TOKEN_NOT_FOUND => {
                write!(
                    f,
                    "[{}] Expect to find \"{}\", not \"{}\"",
                    self.position.clone().unwrap().print_line_column(),
                    self.expected.clone().unwrap(),
                    self.found.clone().unwrap()
                )
            }
            ErrorKind::EXPECTED_PAREN_NOT_FOUND => {
                write!(
                    f,
                    "[{}] Parenthesis failed to match, requires \"{}\", not \"{}\"",
                    self.position.clone().unwrap().print_line_column(),
                    self.expected.clone().unwrap(),
                    self.found.clone().unwrap()
                )
            }
            ErrorKind::EXPECTED_EXPRESSION_NOT_FOUND => {
                write!(
                    f,
                    "[{}] Expect to find an expression, not \"{}\"",
                    self.position.clone().unwrap().print_line_column(),
                    self.found.clone().unwrap()
                )
            }
            ErrorKind::EXPECTED_IDENTIFIER_NOT_FOUND => {
                write!(
                    f,
                    "[{}] Expect to find an identifier, not \"{}\"",
                    self.position.clone().unwrap().print_line_column(),
                    self.found.clone().unwrap()
                )
            }
            ErrorKind::EXPECTED_LITERAL_NOT_FOUND => {
                write!(f, "[{}] Expect to find a literal (Int, Float, Bool, Char, String, Unit), not \"{}\"", self.position.clone().unwrap().print_line_column(), self.found.clone().unwrap())
            }
            ErrorKind::EXPECTED_SEMICOLON_NOT_FOUND => {
                write!(
                    f,
                    "[{}] Expect semicolon ';'",
                    self.position.clone().unwrap().print_line_column()
                )
            }
            ErrorKind::EXPECTED_COMMA_NOT_FOUND => {
                write!(
                    f,
                    "[{}] Expect semicolon ','",
                    self.position.clone().unwrap().print_line_column()
                )
            }
            ErrorKind::EXPECTED_PATTERN_NOT_FOUND => {
                write!(
                    f,
                    "[{}] Expect to find a pattern, not \"{}\"",
                    self.position.clone().unwrap().print_line_column(),
                    self.found.clone().unwrap()
                )
            }
            ErrorKind::EXPECTED_TYPE_PATTERN_NOT_FOUND => {
                write!(
                    f,
                    "[{}] Expect to find a type pattern, not \"{}\"",
                    self.position.clone().unwrap().print_line_column(),
                    self.found.clone().unwrap()
                )
            }
            ErrorKind::EXPECTED_TYPE_NOT_FOUND => {
                write!(
                    f,
                    "[{}] Expect to find a type, not \"{}\"",
                    self.position.clone().unwrap().print_line_column(),
                    self.found.clone().unwrap()
                )
            }
            ErrorKind::EXPECTED_STRUCT_TYPE_ELEMENT_NOT_FOUND => {
                write!(
                    f,
                    "[{}] Expect to find a struct type element, not \"{}\"",
                    self.position.clone().unwrap().print_line_column(),
                    self.found.clone().unwrap()
                )
            }
            ErrorKind::EXPECTED_STRUCT_PATTERN_ELEMENT_NOT_FOUND => {
                write!(
                    f,
                    "[{}] Expect to find a struct pattern element, not \"{}\"",
                    self.position.clone().unwrap().print_line_column(),
                    self.found.clone().unwrap()
                )
            }
            ErrorKind::EXPECTED_STRUCT_TYPE_PATTERN_ELEMENT_NOT_FOUND => {
                write!(
                    f,
                    "[{}] Expect to find a struct type pattern element, not \"{}\"",
                    self.position.clone().unwrap().print_line_column(),
                    self.found.clone().unwrap()
                )
            }
            ErrorKind::EXPECTED_STRUCT_ELEMENT_NOT_FOUND => {
                write!(
                    f,
                    "[{}] Expect to find a struct element, not \"{}\"",
                    self.position.clone().unwrap().print_line_column(),
                    self.found.clone().unwrap()
                )
            }
            ErrorKind::EXPECTED_ENUM_CASE_NOT_FOUND => {
                write!(
                    f,
                    "[{}] Expect to find an enum case, not \"{}\"",
                    self.position.clone().unwrap().print_line_column(),
                    self.found.clone().unwrap()
                )
            }
            ErrorKind::EXPECTED_FUNCTION_ARG_NOT_FOUND => {
                write!(
                    f,
                    "[{}] Expect to find a function argument, not \"{}\"",
                    self.position.clone().unwrap().print_line_column(),
                    self.found.clone().unwrap()
                )
            }
            ErrorKind::EXPECTED_FUNCTION_BLOCK_NOT_FOUND => {
                write!(
                    f,
                    "[{}] Expect to find a function block, not \"{}\"",
                    self.position.clone().unwrap().print_line_column(),
                    self.found.clone().unwrap()
                )
            }
            ErrorKind::EXPECTED_TEMPLATE_DEF_ELEMENT_NOT_FOUND => {
                write!(
                    f,
                    "[{}] Expect to find a template definition element, not \"{}\"",
                    self.position.clone().unwrap().print_line_column(),
                    self.found.clone().unwrap()
                )
            }
            ErrorKind::EXPECTED_DEFINITION_STATEMENT_NOT_FOUND => {
                write!(f, "[{}] Expect to find a definition statement (module, type, shape, operator, bind, export, infix, prefix, postfix)", self.position.clone().unwrap().print_line_column())
            }
            ErrorKind::EXPECTED_IN_BIND_STATEMENT_NOT_FOUND => {
                write!(
                    f,
                    "[{}] Expect to find an in-bind statement (method)",
                    self.position.clone().unwrap().print_line_column()
                )
            }
            ErrorKind::EXPECTED_STATEMENT_NOT_FOUND => {
                write!(
                    f,
                    "[{}] Expect to find a statement",
                    self.position.clone().unwrap().print_line_column()
                )
            }
            ErrorKind::EXPECTED_STATEMENT_BLOCK_NOT_FOUND => {
                write!(
                    f,
                    "[{}] Expect to find a statement block",
                    self.position.clone().unwrap().print_line_column()
                )
            }
            ErrorKind::EXPECTED_METHOD_STATEMENT_IN_IMPLEMENTATION_NOT_FOUND => {
                write!(
                    f,
                    "[{}] Expect to find method statements in implementation statement",
                    self.position.clone().unwrap().print_line_column()
                )
            }
            ErrorKind::EXPECTED_EOF_NOT_FOUND => {
                write!(
                    f,
                    "[{}] Failed to predict EOF (end-of-file)",
                    self.position.clone().unwrap().print_line_column()
                )
            }
            ErrorKind::END_WITHOUT_REACHING_EOF => {
                write!(f, "Parser ended without reaching EOF (UNREACHABLE)")
            }
        }
    }
}

impl ParseError {
    pub fn expected_token_not_found(expected_token_gut: TokenGut, found_token: Token) -> Self {
        Self {
            kind: ErrorKind::EXPECTED_TOKEN_NOT_FOUND,
            position: Some(found_token.position),
            expected: Some(expected_token_gut),
            found: Some(found_token.t_gut),
        }
    }

    pub fn expected_paren_not_found(expected_paren: Paren, found_token: Token) -> Self {
        Self {
            kind: ErrorKind::EXPECTED_PAREN_NOT_FOUND,
            position: Some(found_token.position),
            expected: Some(TokenGut::Paren(expected_paren)),
            found: Some(found_token.t_gut),
        }
    }

    pub fn expected_expression_not_found(found_token: Token) -> Self {
        Self {
            kind: ErrorKind::EXPECTED_EXPRESSION_NOT_FOUND,
            position: Some(found_token.position),
            expected: None,
            found: Some(found_token.t_gut),
        }
    }

    pub fn expected_identifier_not_found(found_token: Token) -> Self {
        Self {
            kind: ErrorKind::EXPECTED_IDENTIFIER_NOT_FOUND,
            position: Some(found_token.position),
            expected: None,
            found: Some(found_token.t_gut),
        }
    }

    pub fn expected_literal_not_found(found_token: Token) -> Self {
        Self {
            kind: ErrorKind::EXPECTED_LITERAL_NOT_FOUND,
            position: Some(found_token.position),
            expected: None,
            found: Some(found_token.t_gut),
        }
    }

    pub fn expected_semicolon_not_found(position: Position) -> Self {
        Self {
            kind: ErrorKind::EXPECTED_SEMICOLON_NOT_FOUND,
            position: Some(position),
            expected: None,
            found: None,
        }
    }
    pub fn expected_comma_not_found(position: Position) -> Self {
        Self {
            kind: ErrorKind::EXPECTED_COMMA_NOT_FOUND,
            position: Some(position),
            expected: None,
            found: None,
        }
    }

    pub fn expected_pattern_not_found(found_token: Token) -> Self {
        Self {
            kind: ErrorKind::EXPECTED_PATTERN_NOT_FOUND,
            position: Some(found_token.position),
            expected: None,
            found: Some(found_token.t_gut),
        }
    }

    pub fn expected_type_pattern_not_found(found_token: Token) -> Self {
        Self {
            kind: ErrorKind::EXPECTED_TYPE_PATTERN_NOT_FOUND,
            position: Some(found_token.position),
            expected: None,
            found: Some(found_token.t_gut),
        }
    }

    pub fn expected_type_not_found(found_token: Token) -> Self {
        Self {
            kind: ErrorKind::EXPECTED_TYPE_NOT_FOUND,
            position: Some(found_token.position),
            expected: None,
            found: Some(found_token.t_gut),
        }
    }

    pub fn expected_struct_type_element_not_found(found_token: Token) -> Self {
        Self {
            kind: ErrorKind::EXPECTED_STRUCT_TYPE_ELEMENT_NOT_FOUND,
            position: Some(found_token.position),
            expected: None,
            found: Some(found_token.t_gut),
        }
    }

    pub fn expected_struct_type_pattern_element_not_found(found_token: Token) -> Self {
        Self {
            kind: ErrorKind::EXPECTED_STRUCT_TYPE_PATTERN_ELEMENT_NOT_FOUND,
            position: Some(found_token.position),
            expected: None,
            found: Some(found_token.t_gut),
        }
    }

    pub fn expected_struct_pattern_element_not_found(found_token: Token) -> Self {
        Self {
            kind: ErrorKind::EXPECTED_STRUCT_PATTERN_ELEMENT_NOT_FOUND,
            position: Some(found_token.position),
            expected: None,
            found: Some(found_token.t_gut),
        }
    }

    pub fn expected_struct_element_not_found(found_token: Token) -> Self {
        Self {
            kind: ErrorKind::EXPECTED_STRUCT_ELEMENT_NOT_FOUND,
            position: Some(found_token.position),
            expected: None,
            found: Some(found_token.t_gut),
        }
    }

    pub fn expected_enum_case_not_found(found_token: Token) -> Self {
        Self {
            kind: ErrorKind::EXPECTED_ENUM_CASE_NOT_FOUND,
            position: Some(found_token.position),
            expected: None,
            found: Some(found_token.t_gut),
        }
    }

    pub fn expected_function_arg_not_found(found_token: Token) -> Self {
        Self {
            kind: ErrorKind::EXPECTED_FUNCTION_ARG_NOT_FOUND,
            position: Some(found_token.position),
            expected: None,
            found: Some(found_token.t_gut),
        }
    }

    pub fn expected_function_block_not_found(found_token: Token) -> Self {
        Self {
            kind: ErrorKind::EXPECTED_FUNCTION_BLOCK_NOT_FOUND,
            position: Some(found_token.position),
            expected: None,
            found: Some(found_token.t_gut),
        }
    }

    pub fn expected_template_def_element_not_found(found_token: Token) -> Self {
        Self {
            kind: ErrorKind::EXPECTED_TEMPLATE_DEF_ELEMENT_NOT_FOUND,
            position: Some(found_token.position),
            expected: None,
            found: Some(found_token.t_gut),
        }
    }

    pub fn expected_definition_statement_not_found(found_token: Token) -> Self {
        Self {
            kind: ErrorKind::EXPECTED_DEFINITION_STATEMENT_NOT_FOUND,
            position: Some(found_token.position),
            expected: None,
            found: None,
        }
    }

    pub fn expected_in_bind_statement_not_found(found_token: Token) -> Self {
        Self {
            kind: ErrorKind::EXPECTED_IN_BIND_STATEMENT_NOT_FOUND,
            position: Some(found_token.position),
            expected: None,
            found: None,
        }
    }

    pub fn expected_statement_block_not_found(found_token: Token) -> Self {
        Self {
            kind: ErrorKind::EXPECTED_STATEMENT_BLOCK_NOT_FOUND,
            position: Some(found_token.position),
            expected: None,
            found: None,
        }
    }

    pub fn expected_statement_not_found(found_token: Token) -> Self {
        Self {
            kind: ErrorKind::EXPECTED_STATEMENT_NOT_FOUND,
            position: Some(found_token.position),
            expected: None,
            found: None,
        }
    }

    pub fn expected_method_statement_in_bind_not_found(found_token: Token) -> Self {
        Self {
            kind: ErrorKind::EXPECTED_METHOD_STATEMENT_IN_IMPLEMENTATION_NOT_FOUND,
            position: Some(found_token.position),
            expected: None,
            found: None,
        }
    }

    pub fn expected_eof_not_found(position: Position) -> Self {
        Self {
            kind: ErrorKind::EXPECTED_EOF_NOT_FOUND,
            position: Some(position),
            expected: None,
            found: None,
        }
    }

    pub fn finish_without_reaching_eof() -> Self {
        Self {
            kind: ErrorKind::END_WITHOUT_REACHING_EOF,
            position: None,
            expected: None,
            found: None,
        }
    }

    pub fn with_tokens(mut self, expected: TokenGut, found: TokenGut) -> Self {
        self.expected = Some(expected);
        self.found = Some(found);
        self
    }
}

pub type ParseErrors = AddableVec<ParseError>;
