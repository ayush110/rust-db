// Initalize modules so we can use functions, types, constants in scope
mod error;
mod meta_command;
mod repl;
mod sql;

use meta_command::handle_meta_command;
use repl::{get_command_type, get_config, CommandType, REPLHelper};
use sql::db::database::Database;
use sql::process_command;

use rustyline::error::ReadlineError;
use rustyline::Editor;

#[macro_use]
extern crate prettytable;

use clap::{crate_name, crate_version};

fn main() -> rustyline::Result<()> {
    env_logger::init();

    // Starting Rustyline with a default configuration
    let config = get_config();

    // Getting a new Rustyline Helper
    let helper = REPLHelper::default();

    // Initiatlizing Rustyline Editor with set config and setting helper
    let mut repl = Editor::with_config(config);
    repl.set_helper(Some(helper));

    // This method loads history file into memory
    // If it doesn't exist, creates one
    // TODO: Check history file size and if too big, clean it.
    if repl.load_history("history").is_err() {
        println!("No previous history.");
    }

    // Friendly intro message for the user
    println!(
        "{} - {}\n{}{}{}{}",
        crate_name!(),
        crate_version!(),
        "Enter .exit to quit.\n",
        "Enter .help for usage hints.\n",
        "Connected to a transient in-memory database.\n",
        "Use '.open FILENAME' to reopen on a persistent database."
    );

    // Creates new database with name and empty hashmap of tables
    let mut db = Database::new("mydb".to_string());

    loop {
        let p = format!("rustdb> ");
        repl.helper_mut().expect("No helper found").colored_prompt =
            format!("\x1b[1;34m{}\x1b[0m", p);
        // Source for ANSI Color information: http://www.perpetualpc.net/6429_colors.html#color_list
        // http://bixense.com/clicolors/

        let readline = repl.readline(&p);
        match readline {
            Ok(command) => {
                repl.add_history_entry(command.as_str());
                // Parsing user's input and returning and enum of repl::CommandType
                match get_command_type(&command.trim().to_owned()) {
                    CommandType::SQLCommand(_cmd) => {
                        // process_command takes care of tokenizing, parsing and executing
                        // the SQL Statement and returning a Result<String, SQLRiteError>
                        let _ = match process_command(&command, &mut db) {
                            Ok(response) => println!("{}", response),
                            Err(err) => eprintln!("An error occured: {}", err),
                        };
                    }
                    CommandType::MetaCommand(cmd) => {
                        // handle_meta_command parses and executes the MetaCommand
                        // and returns a Result<String, SQLRiteError>
                        let _ = match handle_meta_command(cmd, &mut repl) {
                            Ok(response) => println!("{}", response),
                            Err(err) => eprintln!("An error occured: {}", err),
                        };
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                eprintln!("An error occured: {:?}", err);
                break;
            }
        }
    }
    repl.append_history("history").unwrap();

    Ok(())
}
