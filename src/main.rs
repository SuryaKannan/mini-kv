use std::io::{self, Write};

use crate::{engine::process_request, store::MiniKV};
use std::collections::HashMap;

mod store;
mod engine;

fn main() {
    print!("=========================================");
    print!("\nmini-kv - a lightweight in memory DB\n");
    print!("=========================================\n\n");
    
    io::stdout().flush().unwrap();

    let mut input = String::new();

    let mut store = MiniKV{
        store: HashMap::new(),
    };

    loop {

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let cmds = input.trim();

        let res = process_request(cmds, &mut store);

        input.clear();
    }


}

