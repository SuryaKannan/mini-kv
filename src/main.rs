use std::io::{self, Write};
mod storage;
use crate::storage::MiniKV;
mod commands;
use crate::commands::{CommandOutcome,process_request};

fn main() {
    print!("=========================================");
    print!("\nmini-kv - a lightweight in memory DB\n");
    print!("=========================================\n\n");

    io::stdout().flush().unwrap();

    let mut input = String::new();

    let mut mini_kv = MiniKV::new();

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
