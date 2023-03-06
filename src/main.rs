mod engine;
mod helper;
mod salesforce;

use helper::DynError;
use std::env;
use std::io::{self, Write};
use tokio::runtime::Runtime;

fn main() -> Result<(), DynError> {
    println!("Welcome to SOQL Generator");
    loop {
        print!("SOQLGenerator >>> ");
        io::stdout().flush().unwrap();

        let mut expr = String::new();
        io::stdin().read_line(&mut expr).unwrap();

        if expr.trim() == "exit" {
            break;
        }

        let query = engine::build_query(&expr)?;

        let rt = Runtime::new().unwrap();
        rt.block_on(salesforce::run(&query)).unwrap();
    }

    Ok(())
}
