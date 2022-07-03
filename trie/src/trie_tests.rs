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
fn string_present() {
    let mut trie = trie!(("abc", 1));
    let exists = trie.is_exists("abc");

    assert_eq!(exists, true);
}
