mod engine;
mod helper;
mod salesforce;

use helper::DynError;
use std::io::{self, Write};

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

        engine::print(&expr)?;
    }

    Ok(())
}
