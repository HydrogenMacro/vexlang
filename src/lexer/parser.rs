use crate::lexer::input_stream::InputStream;
use std::marker::PhantomData;

use super::span::Span;

struct ErrorInfo {
	span: Span,
	surrounding_lines: Vec<String>,
	error_code: u32
}

enum TokenType {
	Function(Function)
}
struct Function;
struct Token {
	token_type: TokenType,
	lexeme: String,
	span: Span
}
struct ParseError {
	expected_productions: Vec<String>,
	source_idx: usize
}
trait Parse {
	fn parse(&self, input_stream: &mut InputStream) -> Result<Token, ParseError>;
}

pub enum CurrentParseState {
	Function(Parser<Function>)
}
struct Parser<T>(PhantomData<T>);
impl<T> Parser<T> {
	fn new() -> Self {
		Parser(PhantomData::<T>)
	}
}
impl Parse for Parser<Function> {	
	fn parse(&self, input_stream: &mut InputStream) -> Result<Token, ParseError> {
		if input_stream.advance_if_matches_str("fn") {
			
		}
		Err(ParseError { expected_productions: vec!["fn".into()], source_idx: 0  })
	}
}
pub fn parse_next_token(parsers: &'static [Box<dyn Parse>]) {

}

pub fn parse() {
	let input_stream = InputStream::new("abc".into());
	let program_item_parsers = [Parser::<Function>::new()];
	while !input_stream.at_end() {
		//program_item_parsers;
	}
}