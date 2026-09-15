use logos::Logos;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Position {
    pub path: PathBuf,
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new(path: PathBuf, start: usize, end: usize, line: usize, column: usize) -> Self {
        Self {
            path,
            start,
            end,
            line,
            column,
        }
    }

    pub fn from_span(path: PathBuf, span: (usize, usize), input: &str) -> Self {
        let (start, end) = span;
        let line = input[..start].lines().count();
        let column = input[..start].lines().last().map(|l| l.len()).unwrap_or(0) + 1;
        Self {
            path,
            start,
            end,
            line,
            column,
        }
    }

    pub fn span(&self) -> (usize, usize) {
        (self.start, self.end)
    }

    pub fn print_line_column(&self) -> String {
        format!(
            "{}, Line {}, Column {}",
            self.path.display(),
            self.line + 1,
            self.column + 1
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub t_gut: TokenGut,
    pub position: Position,
}

impl Token {
    pub fn new(t_gut: TokenGut, position: Position) -> Self {
        Self { t_gut, position }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub enum LexicalError {
    #[default]
    InvalidToken,
}

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Character(char),
    String(String),
}

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    IntDiv,
    BitAnd,
    BitOr,
    BitXor,
    BitNot,
    BitShiftLeft,
    BitShiftRight,
    Assign,
    Eq,
    Teq,
    Neq,
    Gt,
    Lt,
    Gte,
    Lte,
    And,
    Or,
    Question,
    Exclamation,
    Colon,
    DoubleColon,
    Dot,
    Arrow,
    DoubleArrow,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
    BitAndAssign,
    BitOrAssign,
    BitXorAssign,
    BitShiftLeftAssign,
    BitShiftRightAssign,
    Increment,
    Decrement,
    Comma,
}

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Paren {
    RoundOpen,
    RoundClose,
    SquareOpen,
    SquareClose,
    CurlyOpen,
    CurlyClose,
}

#[derive(Logos, Debug, PartialEq, Clone)]
pub enum Keyword {
    If,
    Else,
    Match,
    Case,
    Then,

    Do,
    While,
    For,
    In,
    Break,
    Continue,

    Return,

    As,
    Module,
    Use,
    Export,

    Let,
    And,

    Try,
    Catch,
    Finally,
    Throw,

    Shape,
    BindDef,
    Operation,
    Coerce,
    Prefix,
    Infix,
    Postfix,
}

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\r]+", error = LexicalError)]
pub enum TokenGut {
    #[regex(r"0[xX][0-9a-fA-F]+", |lex| Literal::Integer(lex.slice().parse::<i64>().unwrap_or(0)))]
    #[regex(r"0[oO][0-7]+", |lex| Literal::Integer(lex.slice().parse::<i64>().unwrap_or(0)))]
    #[regex(r"0[bB][01]+", |lex| Literal::Integer(lex.slice().parse::<i64>().unwrap_or(0)))]
    #[regex(r"\-?0|[1-9][0-9]*", |lex| Literal::Integer(lex.slice().parse::<i64>().unwrap_or(0)))]
    #[regex(r"\-?(0?|[1-9][0-9]*)\.[0-9]+", |lex| Literal::Float(lex.slice().parse::<f64>().unwrap_or(0.0)))]
    #[regex(r"\-?(0?|[1-9])\.[0-9]+[eE][+-]?[0-9]+", |lex| Literal::Float(lex.slice().parse::<f64>().unwrap_or(0.0)))]
    #[token("true", |_| Literal::Boolean(true))]
    #[token("false", |_| Literal::Boolean(false))]
    #[regex(r"'[^']*'", |lex| Literal::Character(lex.slice().chars().nth(1).unwrap()))]
    #[regex(r#""[^"]*""#, |lex| {
        let s = lex.slice();
        Literal::String(s[1..s.len()-1].to_string())
    })]
    Literal(Literal),

    #[token("**", |_| Operator::Pow)]
    #[token("//", |_| Operator::IntDiv)]
    #[token("<<=", |_| Operator::BitShiftLeftAssign)]
    #[token("+=", |_| Operator::AddAssign)]
    #[token("-=", |_| Operator::SubAssign)]
    #[token("*=", |_| Operator::MulAssign)]
    #[token("/=", |_| Operator::DivAssign)]
    #[token("%=", |_| Operator::ModAssign)]
    #[token("&=", |_| Operator::BitAndAssign)]
    #[token("|=", |_| Operator::BitOrAssign)]
    #[token("^=", |_| Operator::BitXorAssign)]
    #[token("<<", |_| Operator::BitShiftLeft)]
    #[token("==", |_| Operator::Eq)]
    #[token("===", |_| Operator::Teq)]
    #[token("!=", |_| Operator::Neq)]
    #[token(">", |_| Operator::Gt)]
    #[token("<", |_| Operator::Lt)]
    #[token("<=", |_| Operator::Lte)]
    #[token("&&", |_| Operator::And)]
    #[token("||", |_| Operator::Or)]
    #[token("::", |_| Operator::DoubleColon)]
    #[token("->", |_| Operator::Arrow)]
    #[token("=>", |_| Operator::DoubleArrow)]
    #[token("+", |_| Operator::Add)]
    #[token("-", |_| Operator::Sub)]
    #[token("*", |_| Operator::Mul)]
    #[token("/", |_| Operator::Div)]
    #[token("%", |_| Operator::Mod)]
    #[token("&", |_| Operator::BitAnd)]
    #[token("|", |_| Operator::BitOr)]
    #[token("^", |_| Operator::BitXor)]
    #[token("~", |_| Operator::BitNot)]
    #[token("=", |_| Operator::Assign)]
    #[token("?", |_| Operator::Question)]
    #[token("!", |_| Operator::Exclamation)]
    #[token(":", |_| Operator::Colon)]
    #[token(".", |_| Operator::Dot)]
    #[token(",", |_| Operator::Comma)]
    Operator(Operator),

    #[token("(", |_| Paren::RoundOpen)]
    #[token(")", |_| Paren::RoundClose)]
    #[token("[", |_| Paren::SquareOpen)]
    #[token("]", |_| Paren::SquareClose)]
    #[token("{", |_| Paren::CurlyOpen)]
    #[token("}", |_| Paren::CurlyClose)]
    Paren(Paren),

    #[token(";")]
    Semicolon,

    #[token("_")]
    WildCard,

    #[token("if", |_| Keyword::If)]
    #[token("else", |_| Keyword::Else)]
    #[token("match", |_| Keyword::Match)]
    #[token("case", |_| Keyword::Case)]
    #[token("then", |_| Keyword::Then)]
    #[token("do", |_| Keyword::Do)]
    #[token("while", |_| Keyword::While)]
    #[token("for", |_| Keyword::For)]
    #[token("in", |_| Keyword::In)]
    #[token("break", |_| Keyword::Break)]
    #[token("continue", |_| Keyword::Continue)]
    #[token("return", |_| Keyword::Return)]
    #[token("as", |_| Keyword::As)]
    #[token("let", |_| Keyword::Let)]
    #[token("and", |_| Keyword::And)]
    #[token("try", |_| Keyword::Try)]
    #[token("catch", |_| Keyword::Catch)]
    #[token("finally", |_| Keyword::Finally)]
    #[token("throw", |_| Keyword::Throw)]
    #[token("shape", |_| Keyword::Shape)]
    #[token("bind", |_| Keyword::BindDef)]
    #[token("operation", |_| Keyword::Operation)]
    #[token("coerce", |_| Keyword::Coerce)]
    #[token("prefix", |_| Keyword::Prefix)]
    #[token("infix", |_| Keyword::Infix)]
    #[token("postfix", |_| Keyword::Postfix)]
    #[token("module", |_| Keyword::Module)]
    #[token("use", |_| Keyword::Use)]
    #[token("export", |_| Keyword::Export)]
    Keyword(Keyword),

    #[regex(r"_+[a-zA-Z0-9][a-zA-Z0-9_]*|[a-zA-Z][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Identifier(String),

    #[regex(r"#[^\n]*", |lex| lex.slice().to_string())]
    Comment(String),

    EOF,
}
