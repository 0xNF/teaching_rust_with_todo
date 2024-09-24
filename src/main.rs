use clap::{Parser, Subcommand};
use rusteria::*;
use std::{io::Write, process::exit};

const DEFAULT_FILE_NAME: &str = "rusteria_todos.json";
const DEFAULT_ERROR_EXIT_CODE: i32 = -1;
const DEFAULT_EXIT_CODE_OK: i32 = 0;

fn main() {
    let mut todos = match load_todos_from_disk() {
        Ok(todos) => todos,
        Err(err) => {
            if err.is_file_not_found_error() {
                let v = Vec::new();
                write_or_quit(&v);
                v
            } else {
                eprintln!("{}", err);
                exit(DEFAULT_ERROR_EXIT_CODE);
            }
        }
    };

    let cli = Cli::parse();
    match cli.command {
        Commands::New { contents } => {
            let added = add_todo(contents, &mut todos);
            println!("Added new TODO: {}", added.id);
            write_or_quit(&todos);
        }
        Commands::Delete { id } => {
            if let Ok(uid) = Uuid::parse_str(&id) {
                match delete_todo(uid, &mut todos) {
                    Ok(()) => {
                        println!("Deleted TODO");
                        write_or_quit(&todos);
                    }
                    Err(e) => {
                        eprintln!("{}", e);
                        exit(DEFAULT_ERROR_EXIT_CODE);
                    }
                }
            } else {
                eprintln!("{}", RusteriaError::InvalidUUID.to_string());
                exit(DEFAULT_ERROR_EXIT_CODE);
            }
        }
        Commands::Update { id, status } => {
            if let Ok(uid) = Uuid::parse_str(&id) {
                let status = match TodoStatus::try_from(status) {
                    Ok(status) => status,
                    Err(e) => {
                        eprintln!("{}", e);
                        exit(DEFAULT_ERROR_EXIT_CODE);
                    }
                };

                if let Err(e) = update_todo(uid, status, &mut todos) {
                    eprintln!("{}", e);
                    exit(DEFAULT_ERROR_EXIT_CODE);
                } else {
                    println!("Updated TODO");
                    write_or_quit(&todos);
                }
            } else {
                eprintln!("{}", RusteriaError::InvalidUUID.to_string());
                exit(DEFAULT_ERROR_EXIT_CODE);
            }
        }
        Commands::List {
            open_closed_both,
            verbose,
        } => {
            let filter = TodoFilter {
                status: open_closed_both.map(TodoStatus::from),
            };
            let filtered = list_todos(filter, todos);
            println!("{} todo items", filtered.len());
            for todo in filtered {
                println!("{}", todo);
                if verbose {
                    println!("({})", todo.id);
                }
            }
        }
    }
    exit(DEFAULT_EXIT_CODE_OK);
}

/// Writes the list of Todo Items to disk.
///
/// If the write fails for whatever reason, the error is printed to the stderr, and the application immediately terminates with an error code
fn write_or_quit(items: &Vec<TodoItem>) {
    if let Err(err) = write_todos_to_disk(items) {
        eprintln!("Failed to write ToDo json to disk: {}", err);
        exit(DEFAULT_ERROR_EXIT_CODE);
    }
}

fn write_todos_to_disk(items: &Vec<TodoItem>) -> Result<(), RusteriaError> {
    let json = serde_json::to_string_pretty(items)
        .map_err(|e| RusteriaError::Unknown(format!("Failed to make json: {}", e)))?;
    let mut f = std::fs::File::create(DEFAULT_FILE_NAME)
        .map_err(|e| RusteriaError::Unknown(format!("Failed to create file: {}", e)))?;
    f.write_all(json.as_bytes())
        .map_err(|e| RusteriaError::Unknown(format!("Failed to write file: {}", e)))
}

fn load_todos_from_disk() -> Result<Vec<TodoItem>, RusteriaError> {
    let file = std::fs::File::open(DEFAULT_FILE_NAME).map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            RusteriaError::NoTodoFile(DEFAULT_FILE_NAME.into())
        } else {
            RusteriaError::Unknown("Unknown file error".into())
        }
    })?;
    let reader = std::io::BufReader::new(file);
    let data: Vec<TodoItem> =
        serde_json::from_reader(reader).map_err(|e| RusteriaError::Unknown(e.to_string()))?;

    Ok(data)
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Create a new TODO item")]
    New { contents: String },
    #[command(about = "Update the status of a TODO item by ID")]
    Update { id: String, status: String },
    #[command(about = "Show all TODOs")]
    List {
        open_closed_both: Option<bool>,
        #[arg(short, long, default_value_t = false)]
        verbose: bool,
    },
    #[command(about = "Deletes the specified TODO item")]
    Delete { id: String },
}
