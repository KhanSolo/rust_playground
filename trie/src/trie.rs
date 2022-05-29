use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

#[derive(Debug)]
struct Node {
     value: Option<u32>,
     max_child_value: Option<u32>,
     children: HashMap<char, Node>,
}

#[derive(Debug)]
pub struct Trie {
    head: Node,
}

impl Trie {
    pub fn new() -> Self {
        Trie { 
            head : Node {
                value : None,
                max_child_value : None,
                children : HashMap::new(),
            }
        }
    }

    pub fn add(&mut self, string: String, value:u32){
        let h = &self.head;
        for c in string.chars() {
            //println!("{}", c);
        }
    }
}

