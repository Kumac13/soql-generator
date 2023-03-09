mod engine;
mod helper;
mod salesforce;

use helper::DynError;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use tokio::runtime::Runtime;

use crate::salesforce::Connection;

fn main() -> Result<(), DynError> {
    let mut rl = DefaultEditor::new()?;
    #[cfg(feature = "with-file-history")]
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    let rt = Runtime::new().unwrap();
    let conn = rt.block_on(async {
        let conn = Connection::new().await?;
        Ok::<Connection, Box<dyn std::error::Error + Send + Sync>>(conn)
    })?;

    println!("Welcome to SOQL Generator");
    println!("Type 'exit' to quit");
    loop {
        let readline = rl.readline("SOQLGenerator >>> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                if line.trim() == "exit" {
                    break;
                }

                let (query, open_browser) = match engine::build_query(&line) {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("{}", e);
                        continue;
                    }
                };

                rt.block_on(conn.call_query(&query, open_browser)).unwrap();
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    #[cfg(feature = "with-file-history")]
    rl.save_history("history.txt").unwrap_or_else(|e| {
        eprintln!("Failed to save history: {e}");
    });

    Ok(())
}
