mod trie;

use trie::Trie;

fn main() {
    let mut trie = Trie::new();

    trie.add("abc".to_owned(), 1);

    println!("{:#?}", trie);
}
