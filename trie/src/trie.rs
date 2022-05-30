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

fn moving<T>(t:T)->T{t}

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

        let node = &mut self.head;
        for c in string.chars() {

            let mut nd =
                match moving(node).children.get_key_value(&c) {
                    Some((_, mut n)) => {
                        println!("Some {:?}", n);
                        n
                    },
                    None => {
                        let new_node = Node {
                            value:None,
                            max_child_value: None,
                            children : HashMap::new(),
                        };
                        let childs = &mut node.children;
                        childs.insert(c, new_node);
                        &mut new_node
                    },
                };
                node = nd;
        }
        node.value = Some(value);
    }
}
