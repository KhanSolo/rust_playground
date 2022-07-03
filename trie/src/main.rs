mod trie;

#[cfg(test)]
mod trie_tests;

use crate::trie::Trie;

fn main() {
    let trie = trie!(
        ("kare", 10),
        ("kanojo", 20),
        ("karetachi", 1),
        ("korosu", 7),
        ("sakura", 3)
    );

    //println!("{:#?}", trie);

    let response = trie.prefix("k");
    for line in response {
        println!("{}", line);
    }
}
