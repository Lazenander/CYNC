use crate::compiler::lexer::lexer::{lex, LexResult};
use crate::compiler::parser::error::ParseErrors;
use crate::compiler::parser::parser::{Parsed, ParsedBox, Parser};
use crate::compiler::parser::program::program::Program;
use crate::pipeline::driver::driver::CompilerDriver;
use crate::pipeline::driver::error::PipelineError;
use crate::utility::common::vec_add::AddableVec;
use crate::utility::consts::{CELL_DEPENDENCIES_DIR_NAME, CYNC_FILE_EXTENSION};
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use walkdir::WalkDir;

#[derive(Debug)]
pub enum FileParsedResult {
    Cync(CyncFileParsedResult),
}

#[derive(Debug)]
pub struct CyncFileParsedResult {
    pub file_path: PathBuf,

    pub lex_result: LexResult,
    pub lex_duration: Duration,
    pub parse_result: ParsedBox<Program>,
    pub parse_errors: ParseErrors,
    pub parse_duration: Duration,
}

impl FileParsedResult {
    pub fn new_cync_file_analysis(file_path: PathBuf) -> Self {
        let content = fs::read_to_string(file_path.clone()).unwrap();
        let lex_start = Instant::now();
        let lex_result = lex(&content, file_path.clone());
        let lex_duration = lex_start.elapsed();
        let mut parser = Parser::new(lex_result.tokens.clone());
        let parse_start = Instant::now();
        let parse_result = parser.parse_cync();
        let parse_duration = parse_start.elapsed();

        FileParsedResult::Cync(CyncFileParsedResult {
            file_path,
            lex_result,
            lex_duration,
            parse_result: parse_result.value,
            parse_errors: parse_result.error,
            parse_duration,
        })
    }
}
