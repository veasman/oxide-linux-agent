use std::env;
use std::io::{self, Write};

mod commands;

#[derive(Debug)]
enum Command {
    Help,
    About,
    Exit,
    Factors(Option<String>),
    HealthCheck,
    Logs,
    Reset,
    Settings { key: Option<String>, value: Option<String> },
    Status,
    Sync(Option<String>),
    Unknown(String),
}

impl From<&str> for Command {
    fn from(input: &str) -> Self {
        let parts: Vec<&str> = input.split_whitespace().collect();
        let cmd = parts.get(0).map(|s| s.to_lowercase());

        match cmd.as_deref() {
            Some("help") => Command::Help,
            Some("about") => Command::About,
            Some("exit") => Command::Exit,
            Some("factors") => Command::Factors(parts.get(1).map(|s| s.to_string())),
            Some("healthcheck") => Command::HealthCheck,
            Some("logs") => Command::Logs,
            Some("reset") => Command::Reset,
            Some("settings") => {
                Command::Settings {
                    key: parts.get(1).map(|s| s.to_string()),
                    value: parts.get(2).map(|s| s.to_string()),
                }
            }
            Some("status") => Command::Status,
            Some("sync") => Command::Sync(parts.get(1).map(|s| s.to_string())),
            Some(unknown) => Command::Unknown(unknown.to_string()),
            None => Command::Unknown("".to_string()),
        }
    }
}

fn handle_command(command: Command) {
    match command {
        Command::Help => commands::help::show_commands(),
        Command::About => commands::about::show_user_info(),
        Command::Exit => std::process::exit(0),
        Command::Factors(arg) => match arg {
            Some(arg_value) => println!("Factors command with arg: {}", arg_value),
            None => println!("Factors command without arg"),
        },
        Command::HealthCheck => println!("HealthCheck command"),
        Command::Logs => println!("Logs command"),
        Command::Reset => println!("Reset command"),
        Command::Settings { key, value } => {
            match (key, value) {
                (Some(key), Some(value)) => println!("Settings command with key: {}, value: {}", key, value),
                (Some(key), None) => println!("Settings command with key: {}, no value provided", key),
                (None, _) => println!("Settings command without key"),
            }
        }
        Command::Status => println!("Status command"),
        Command::Sync(arg) => println!("Sync command with arg: {:?}", arg),
        Command::Unknown(cmd) => println!("Unknown command: {}", cmd),
    }
}

fn main() {
    println!("Oxide Linux Agent v19.0.2");

    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        loop {
            print!(">> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();

            match io::stdin().read_line(&mut input) {
                Ok(0) => break, // Ctrl-C
                Ok(_) => {
                    input = input.trim().to_string();
                    let command = Command::from(input.as_str());
                    handle_command(command);
                }
                Err(error) => {
                    eprintln!("Error reading input: {}", error);
                    break;
                }
            }
        }
    } else {
        let input = args.join(" ");
        let command = Command::from(input.as_str());
        handle_command(command);
    }
}
