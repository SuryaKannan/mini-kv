use std::io::{self, Write};

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct MiniKV {
    pub store: HashMap<String, String>,
}

enum Command {
    Get,
    Set,
    Delete,
    Exit,
    Invalid,
}

enum CommandOutcome {
    Msg(String),
    ExitSignal,
}

impl Command {
    fn from_str(s: &str) -> Command {
        match s {
            "GET" => Command::Get,
            "SET" => Command::Set,
            "DELETE" => Command::Delete,
            "EXIT" => Command::Exit,
            _ => Command::Invalid,
        }
    }

    fn expected_tokens(&self) -> usize {
        match self {
            Command::Set => 3,                   // set key value pair
            Command::Get | Command::Delete => 2, // get or delete based on key only
            Command::Exit => 1,
            _ => 0,
        }
    }

    fn execute(self, args: Vec<String>, db: &mut MiniKV) -> CommandOutcome {
        let expected_tokens = self.expected_tokens();

        if !matches!(self, Command::Invalid) && args.len() != expected_tokens {
            return CommandOutcome::Msg(format!(
                "Expected {} args, including the command\n",
                expected_tokens
            ));
        }

        match self {
            Command::Set => self.set_cmd(args, db),
            Command::Get => self.get_cmd(args, db),
            Command::Delete => self.delete_cmd(args, db),
            Command::Exit => self.exit_cmd(),
            Command::Invalid => self.invalid_cmd(),
        }
    }

    fn set_cmd(&self, args: Vec<String>, db: &mut MiniKV) -> CommandOutcome {
        let mut parts = args.into_iter().skip(1);

        let key = parts.next().expect("SET argument count already validated");
        let value = parts.next().expect("SET argument count already validated");

        db.store.insert(key, value);

        CommandOutcome::Msg("executed set!\n".to_string())
    }

    fn get_cmd(&self, args: Vec<String>, db: &MiniKV) -> CommandOutcome {
        let mut parts = args.into_iter().skip(1);

        let key = parts.next().expect("GET argument count already validated");

        match db.store.get(&key) {
            Some(value) => CommandOutcome::Msg(value.clone()),
            None => CommandOutcome::Msg(format!("Key '{key}' not found")),
        }
    }

    fn delete_cmd(&self, args: Vec<String>, db: &mut MiniKV) -> CommandOutcome {
        let mut parts = args.into_iter().skip(1);

        let key = parts
            .next()
            .expect("DELETE argument count already validated");

        match db.store.remove(&key) {
            Some(_) => CommandOutcome::Msg(format!("Key `{key}` successfully removed")),
            None => CommandOutcome::Msg(format!("Key '{key}' not found")),
        }
    }

    fn exit_cmd(&self) -> CommandOutcome {
        CommandOutcome::ExitSignal
    }

    fn invalid_cmd(&self) -> CommandOutcome {
        CommandOutcome::Msg("invalid command!\n".to_string())
    }
}

fn main() {
    print!("=========================================");
    print!("\nmini-kv - a lightweight in memory DB\n");
    print!("=========================================\n\n");

    io::stdout().flush().unwrap();

    let mut input = String::new();

    let mut mini_kv = MiniKV {
        store: HashMap::new(),
    };

    loop {
        input.clear();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let cmds = input.trim();

        let end_command = process_request(cmds, &mut mini_kv);

        match end_command {
            CommandOutcome::Msg(value) => println!("{value}\n"),
            CommandOutcome::ExitSignal => {
                println!("\nEnding Session...\n");
                break;
            }
        }
    }
}

/// Entrypoint for processing a user query
///
/// Right now we support SET, GET, DELETE and EXIT commands
fn process_request(cmds: &str, db: &mut MiniKV) -> CommandOutcome {
    let cmd_array = parse_cmds(cmds);

    let cmd = Command::from_str(&cmd_array[0]);

    cmd.execute(cmd_array, db)
}

/// Convert raw input string into a vector of commands
fn parse_cmds(cmds: &str) -> Vec<String> {
    let mut cmd_array: Vec<String> = Vec::new();

    let mut prev_idx = 0;

    // push all the commands with spaces between them
    for (idx, c) in cmds.chars().enumerate() {
        if c == ' ' {
            cmd_array.push(cmds[prev_idx..idx].to_string());
            prev_idx = idx + 1;
        }
    }

    // push final command
    cmd_array.push(cmds[prev_idx..].to_string());

    cmd_array
}
