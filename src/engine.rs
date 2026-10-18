enum Command {
    Get,
    Set,
    Delete,
    Exit,
    Invalid
}

impl Command {
    fn from_str(s: &str) -> Command{
        match s {
            "GET" => Command::Get,
            "SET" => Command::Set,
            "DELETE" => Command::Delete,
            "EXIT" => Command::Exit,
            _ => Command::Invalid
        }
    }

    fn execute(self) {
        match self {
            Command::Get => println!("Fetching data..."),
            Command::Set => println!("Saving data..."),
            Command::Delete => println!("Removing data..."),
            Command::Exit => println!("Exiting program..."),
            Command::Invalid => println!("Invalid command..."),
        }
    }
}

pub fn mini_kv(cmds: &str) -> Result<bool, String> {

    let cmd_array = parse_cmds(cmds);
    let cmd = Command::from_str(&cmd_array[0]);

    cmd.execute();
    
    Ok(true)
}

fn parse_cmds(cmds: &str) -> Vec<String> {

    let mut cmd_array: Vec<String> = Vec::new();

    let mut prev_idx = 0;

    // push all the commands with spaces between them
    for (idx, c) in cmds.chars().enumerate(){
        if c == ' ' {
            cmd_array.push(cmds[prev_idx..idx].to_string());
            prev_idx = idx + 1;
        }
    }

    // push final command
    cmd_array.push(cmds[prev_idx..].to_string());

    cmd_array

}