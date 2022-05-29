use std::collections::HashMap;

#[derive(Debug)]
pub struct Node {
    pub value: Option<u16>,
    pub children: HashMap<char, Node>,
}

#[derive(Debug)]
pub struct Trie {
    pub Head: Node,
}

impl Trie {
    pub fn new() -> Self {
        Trie { 
            Head : Node {
                value : None,
                children : HashMap::new(),
            }
        }
    }
}