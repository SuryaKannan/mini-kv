use crate::store::MiniKV;

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

    fn execute(self, cmds: Vec<String>, store: &mut MiniKV) -> Result<bool, String>{
        match self {
            Command::Get => self.get_cmd(&cmds, store),
            Command::Set => self.set_cmd(&cmds, store),
            Command::Delete => self.delete_cmd(&cmds,store),
            Command::Exit => self.exit_cmd(&cmds,store),
            Command::Invalid => self.invalid_cmd(&cmds,store),
        }
    }

    fn get_cmd(&self, cmds: &Vec<String>, store: &mut MiniKV) -> Result<bool, String> {
        println!("{cmds:?}");
        Ok(true)
    }

    fn set_cmd(&self, cmds: &Vec<String>, store: &mut MiniKV) -> Result<bool, String> {
        println!("{cmds:?}");
        store.store.insert(<String as Clone>::clone(&cmds[1]), <String as Clone>::clone(&cmds[2]));
        Ok(true)
    }

    fn delete_cmd(&self, cmds: &Vec<String>, store: &mut MiniKV) -> Result<bool, String> {
        println!("{cmds:?}");
        Ok(true)
    }

    fn exit_cmd(&self, cmds: &Vec<String>, store: &mut MiniKV) -> Result<bool, String> {
        println!("{cmds:?}");
        Ok(true)
    }

    fn invalid_cmd(&self, cmds: &Vec<String>, store: &mut MiniKV) -> Result<bool, String> {
        println!("{cmds:?}");
        Ok(true)
    }
}

pub fn process_request(cmds: &str, store: &mut MiniKV) -> Result<bool, String> {

    let cmd_array = parse_cmds(cmds);
    let cmd = Command::from_str(&cmd_array[0]);

    cmd.execute(cmd_array, store);

    println!("{store:?}");
    
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