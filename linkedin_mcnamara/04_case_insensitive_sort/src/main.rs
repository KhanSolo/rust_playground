fn main() {
    let mut users = vec!["Todd","amy"];
    sort_usernames(&mut users);
    assert_eq!(users, vec!["amy", "Todd"]);
}

fn sort_usernames<T:AsRef<str>>(users:&mut Vec<T>){
    //users.sort_by(|a, b| a.as_ref().cmp(b.as_ref()));
    users.sort_by_key(|s| s.as_ref().to_lowercase()); // to_lowercase() allocates memory
}