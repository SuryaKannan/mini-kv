use std::collections::HashMap;

#[derive(Debug)]
pub struct MiniKV {
    pub store: HashMap<String,String>,
}