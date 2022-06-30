mod trie;

use trie::Trie;

fn main() {
    let mut trie = Trie::new();

    trie.add("kare".to_owned(), 10);
    trie.add("kanojo".to_owned(), 20);
    trie.add("karetachi".to_owned(), 1);
    trie.add("korosu".to_owned(), 7);
    trie.add("sakura".to_owned(), 3);

    println!("{:#?}", trie);

    let response = trie.get("k".to_owned());
    for line in response {
        println!("{}", line);
    }
}
