mod ast;
mod lexer;
mod parse;
mod querygen;
mod token;

use crate::engine::lexer::tokenize;
use crate::engine::parse::Parser;
use crate::engine::querygen::Query;
use crate::helper::DynError;

pub fn build_query(expr: &str) -> Result<(String, bool), DynError> {
    let tokens = tokenize(expr);
    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;

    let mut query = Query::default();
    query.evaluate(program)?;
    let generated_code = query.generate();

    println!("generated query: {}", generated_code);

    Ok((generated_code, query.open_browser))
}
