mod engine;
mod helper;
mod hint;
mod salesforce;

use crate::salesforce::Connection;
use clap::Parser;
use helper::DynError;
use hint::QueryHinter;
use rustyline::error::ReadlineError;
use rustyline::history::DefaultHistory;
use rustyline::Editor;

/// Tool for interactively executing SOQL queries
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// query for std out mode
    #[arg(short, long)]
    query: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), DynError> {
    let args = Args::parse();

    if let Some(query) = args.query {
        let conn = Connection::new().await?;
        let (parsed_query, _open_browser) = engine::build_query(&query)?;
        conn.call_query(&parsed_query, false).await?;
    } else {
        run().await?;
    }

    Ok(())
}

async fn run() -> Result<(), DynError> {
    let mut conn = Connection::new().await?;
    conn.get_objects().await?;

    let hinter = QueryHinter::new(&conn);

    let mut rl: Editor<QueryHinter, DefaultHistory> = Editor::new()?;
    rl.set_helper(Some(hinter));

    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    println!("Welcome to SOQL Generator");
    println!("Type 'exit' to quit");
    loop {
        let readline = rl.readline("SOQLGenerator >>> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;

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

                conn.call_query(&query, open_browser).await?;
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

    if let Err(e) = rl.save_history("history.txt") {
        eprintln!("Failed to save history: {}", e);
    }

    Ok(())
}
