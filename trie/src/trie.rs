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

    pub fn add_tuple(&mut self, (a, b): (&str, u32)) {
        self.add(a, b);
    }

    pub fn add(&mut self, string: &str, value: u32) {
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

    pub fn is_exists(&mut self, entry: &str) -> bool {
        let mut node = &mut self.head;
        for c in entry.chars() {
            node = match node.children.entry(c) {
                Entry::Occupied(n) => n.into_mut(),
                Entry::Vacant(_) => return false,
            };
        }
        match node.value {
            Some(_) => true,
            None => false,
        }
    }

    pub fn prefix(&mut self, query: &str) -> Vec<(&str, u32)> {
        let mut result = Vec::new();

        if !query.is_empty() {
            let mut node = &mut self.head;
            for c in query.chars() {
                node = match node.children.entry(c) {
                    Entry::Occupied(n) => n.into_mut(),
                    Entry::Vacant(a) => {
                            todo!()
                    },
                };
            }

        }
        result
    }
}

#[macro_export]
macro_rules! trie {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Trie::new();
            $(
                temp_vec.add_tuple($x);
            )*
            temp_vec
        }
    };
}
