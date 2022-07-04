use super::*;

#[test]
fn string_absent() {
    let mut trie = trie!(("abc", 1));
    let exists = trie.is_exists("bcd");

    assert_eq!(exists, false);
}

#[test]
fn string_absent_substr() {
    let mut trie = trie!(("abcdef", 1));
    let exists = trie.is_exists("abc");

    assert_eq!(exists, false);
}

#[test]
fn string_exists() {
    let mut trie = trie!(("abc", 1));
    let exists = trie.is_exists("abc");

    assert_eq!(exists, true);
}

#[test]
fn prefix_notfound(){
    let mut trie = trie!(("abc", 1));
    let found = trie.prefix("bcd");

    assert!(found.is_empty());
}

#[test]
fn prefix_found_single(){
    let mut trie = trie!(("abc", 1));
    let found = trie.prefix("a");

    assert_eq!(found.len(), 1);
}

#[test]
fn prefix_found_suitable(){
    let trie = trie!(
        ("kare", 10),
        ("kanojo", 20),
        ("karetachi", 1),
        ("korosu", 7),
        ("sakura", 3)
    );
    let found = trie.prefix("k");

    assert_eq!(found.len(), 4);
}