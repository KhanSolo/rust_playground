use std::collections::{hash_map::Entry, HashMap};
use std::fmt::Debug;

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

//fn moving<T>(t:T)->T{t}

impl Trie {
    pub fn new() -> Self {
        Trie {
            head: Node {
                value: None,
                max_child_value: None,
                children: HashMap::new(),
            },
        }
    }

    pub fn add(&mut self, string: String, value: u32) {
        let mut node = &mut self.head;
        for c in string.chars() {
            node = match node.children.entry(c) {
                Entry::Occupied(entry) => {
                    let m = entry.into_mut();
                    if m.max_child_value < Some(value) {
                        m.max_child_value = Some(value);
                    }
                    m
                }
                Entry::Vacant(entry) => entry.insert(Node {
                    value: None,
                    max_child_value: Some(value),
                    children: HashMap::new(),
                }),
            };
        }
        node.value = Some(value);
        node.max_child_value = Some(value);
    }
}
