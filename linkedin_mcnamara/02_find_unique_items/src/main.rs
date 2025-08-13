use std::{collections::HashSet, hash::Hash};

fn main() {
    let before = vec![4,1,2,1,3,1,4];
    println!("Before:");
    for e in &before {
        println!("{e}");
    }
    println!("After:");
    let after = unique(before);
    for e in &after {
        println!("{e}");
    }
}

fn unique<T:Ord + Hash + Copy>(vec:Vec<T>) -> Vec<T> {
    let mut hash = HashSet::new();
    let mut res = vec![];
    for e in vec{
        if hash.contains(&e) {continue;}
        res.push(e);
        hash.insert(e);
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unique_empty_vec() {
        let vec: Vec<i32> = vec![];
        let after = unique(vec);
        assert_eq!(Vec::new() as Vec<i32>, after);
    }

    #[test]
    fn unique_some_dups() {
        let vec: Vec<i32> = vec![1,1,2,1,3,1,4];
        let after = unique(vec);
        assert_eq!(vec![1,2,3,4], after);
    }

    #[test]
    fn unique_retains() {
        let vec: Vec<i32> = vec![4,1,2,1,3,1,4];
        let after = unique(vec);
        assert_eq!(vec![4,1,2,3], after);
    }
}