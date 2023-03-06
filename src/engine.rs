mod parser;
mod querygen;

use crate::helper::DynError;

pub fn print(expr: &str) -> Result<(), DynError> {
    println!("expr: {expr}");
    let query = parser::parse(expr)?;
    println!("query: {:?}", query);

    println!();
    println!("generated query:");
    let generated_code = query.generate();
    println!("{}", generated_code);

    Ok(())
}

pub fn build_query(expr: &str) -> Result<(String, bool), DynError> {
    let query = parser::parse(expr)?;
    let generated_code = query.generate();

    Ok((generated_code, query.open_browser))
}
