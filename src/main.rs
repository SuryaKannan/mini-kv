use std::io::{self, Write};

use crate::engine::mini_kv;

mod engine;

fn main() {
    print!("=========================================");
    print!("\nMINI-KV - a lightweight in memory kv DB\n");
    print!("=========================================\n\n");
    
    io::stdout().flush().unwrap();

    let mut input = String::new();

    loop {

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let cmds = input.trim();

        let res = mini_kv(cmds);

        input.clear();
    }


}

