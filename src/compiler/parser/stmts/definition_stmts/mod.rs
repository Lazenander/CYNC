pub use crate::compiler::lexer::tokens::{Keyword, Paren, TokenGut};
pub use crate::compiler::parser::error::ParseError;
pub use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};
use crate::compiler::parser::stmts::definition_stmts::bind::BindDef;
use crate::compiler::parser::stmts::definition_stmts::coerce::Coerce;
use crate::compiler::parser::stmts::definition_stmts::operation::OperationDef;
use crate::compiler::parser::stmts::definition_stmts::operator::OperatorOverload;
use crate::compiler::parser::stmts::definition_stmts::r#use::Use;
use crate::compiler::parser::stmts::definition_stmts::shape::ShapeDef;
pub use module::Module;

pub mod bind;
pub mod coerce;
pub mod export;
pub mod module;
pub mod operation;
pub mod operator;
pub mod shape;
pub mod r#use;

#[derive(Debug, Clone)]
pub enum DefinitionStmt {
    Empty,
    SubModule(ParsedBox<Module>),
    Shape(ParsedBox<ShapeDef>),
    Operation(ParsedBox<OperationDef>),
    BindDef(ParsedBox<BindDef>),
    Coerce(ParsedBox<Coerce>),
    Operator(ParsedBox<OperatorOverload>),
    Use(ParsedBox<Use>),
}

const DEFINITION_STMT_SYNC_SET: [TokenGut; 13] = [
    TokenGut::Keyword(Keyword::Module),
    TokenGut::Keyword(Keyword::Shape),
    TokenGut::Keyword(Keyword::Operation),
    TokenGut::Keyword(Keyword::BindDef),
    TokenGut::Keyword(Keyword::Infix),
    TokenGut::Keyword(Keyword::Prefix),
    TokenGut::Keyword(Keyword::Postfix),
    TokenGut::Keyword(Keyword::Coerce),
    TokenGut::Keyword(Keyword::Use),
    TokenGut::Keyword(Keyword::Export),
    TokenGut::Semicolon,
    TokenGut::Paren(Paren::CurlyClose),
    TokenGut::EOF,
];

#[derive(Debug, Clone)]
pub struct DefinitionStmts(pub Vec<ParsedBox<DefinitionStmt>>);

impl Parser {
    pub fn parse_definition_stmts(&mut self) -> Parsed<DefinitionStmts> {
        Parsed::lift_parsed(
            self.parse_star(|c_self| {
                let token = c_self.current_token()?;

                if token.t_gut != TokenGut::Paren(Paren::CurlyClose) && token.t_gut != TokenGut::EOF
                {
                    let p_stmt = c_self.parse_definition_stmt();
                    return Some(p_stmt);
                }
                return None;
            }),
            |v| DefinitionStmts(v),
        )
    }

    pub fn parse_definition_stmt(&mut self) -> Parsed<DefinitionStmt> {
        self.force_parse_with_default(
            DefinitionStmt::Empty,
            Parser::try_parse_definition_stmt,
            ParseError::expected_definition_statement_not_found,
            DEFINITION_STMT_SYNC_SET.into(),
        )
    }

    pub fn try_parse_definition_stmt(&mut self) -> Option<Parsed<DefinitionStmt>> {
        self.safe_try(|s_self| {
            let token = s_self.current_token().unwrap();
            match token.t_gut {
                TokenGut::Keyword(Keyword::Module) => {
                    Some(s_self.parse_sub_module_definition_stmt())
                }
                TokenGut::Keyword(Keyword::Shape) => Some(s_self.parse_shape_definition_stmt()),
                TokenGut::Keyword(Keyword::Operation) => {
                    Some(s_self.parse_operation_definition_stmt())
                }
                TokenGut::Keyword(Keyword::BindDef) => Some(s_self.parse_bind_definition_stmt()),
                TokenGut::Keyword(Keyword::Infix)
                | TokenGut::Keyword(Keyword::Prefix)
                | TokenGut::Keyword(Keyword::Postfix) => {
                    Some(s_self.parse_operator_definition_stmt())
                }
                TokenGut::Keyword(Keyword::Coerce) => Some(s_self.parse_coerce_definition_stmt()),
                TokenGut::Keyword(Keyword::Use) => Some(s_self.parse_use_definition_stmt()),
                TokenGut::Keyword(Keyword::Export) => match s_self.next_token().unwrap().t_gut {
                    TokenGut::Keyword(Keyword::Module) => {
                        Some(s_self.parse_sub_module_definition_stmt())
                    }
                    TokenGut::Keyword(Keyword::Shape) => Some(s_self.parse_shape_definition_stmt()),
                    TokenGut::Keyword(Keyword::Operation) => {
                        Some(s_self.parse_operation_definition_stmt())
                    }
                    TokenGut::Keyword(Keyword::Use) => Some(s_self.parse_use_definition_stmt()),
                    TokenGut::Keyword(Keyword::BindDef) => {
                        Some(s_self.parse_bind_definition_stmt())
                    }
                    _ => Some(s_self.parse_export_definition_stmt()),
                },
                TokenGut::Semicolon => Some(s_self.parse_empty_definition_stmt()),
                _ => None,
            }
        })
    }
}

impl Parser {
    pub fn parse_empty_definition_stmt(&mut self) -> Parsed<DefinitionStmt> {
        let p_sc = self.parse_semicolon();
        Parsed::lift_parsed(p_sc, |_| DefinitionStmt::Empty)
    }

    pub fn parse_sub_module_definition_stmt(&mut self) -> Parsed<DefinitionStmt> {
        let p_module = Parsed::box_parsed(self.parse_module());
        Parsed::lift_parsed(p_module, |module| DefinitionStmt::SubModule(module))
    }

    pub fn parse_shape_definition_stmt(&mut self) -> Parsed<DefinitionStmt> {
        let p_shape_def = Parsed::box_parsed(self.parse_shape_def());
        Parsed::lift_parsed(p_shape_def, |shape_def| DefinitionStmt::Shape(shape_def))
    }

    pub fn parse_operation_definition_stmt(&mut self) -> Parsed<DefinitionStmt> {
        let p_operation_def = Parsed::box_parsed(self.parse_operation_def());
        Parsed::lift_parsed(p_operation_def, |operation_def| {
            DefinitionStmt::Operation(operation_def)
        })
    }

    pub fn parse_bind_definition_stmt(&mut self) -> Parsed<DefinitionStmt> {
        let p_bind = Parsed::box_parsed(self.parse_bind());
        Parsed::lift_parsed(p_bind, |bind| DefinitionStmt::BindDef(bind))
    }

    pub fn parse_operator_definition_stmt(&mut self) -> Parsed<DefinitionStmt> {
        let p_operator_overload = Parsed::box_parsed(self.parse_operator_overload_def());
        Parsed::lift_parsed(p_operator_overload, |operator_overload| {
            DefinitionStmt::Operator(operator_overload)
        })
    }

    pub fn parse_coerce_definition_stmt(&mut self) -> Parsed<DefinitionStmt> {
        let p_coerce_def = Parsed::box_parsed(self.parse_coerce_def());
        Parsed::lift_parsed(p_coerce_def, |coerce_def| {
            DefinitionStmt::Coerce(coerce_def)
        })
    }

    pub fn parse_use_definition_stmt(&mut self) -> Parsed<DefinitionStmt> {
        let p_use = Parsed::box_parsed(self.parse_use());
        Parsed::lift_parsed(p_use, |use_stmt| DefinitionStmt::Use(use_stmt))
    }

    pub fn parse_export_definition_stmt(&mut self) -> Parsed<DefinitionStmt> {
        let p_export = Parsed::box_parsed(self.parse_export());
        Parsed::lift_parsed(p_export, |export| DefinitionStmt::Use(export))
    }
}
