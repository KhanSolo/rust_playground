mod trie;

use trie::Trie;

fn main() {
    let mut trie = Trie::new();

    trie.add("abcde".to_owned(), 1);
    trie.add("abcfe".to_owned(), 1);

    println!("{:#?}", trie);
}
