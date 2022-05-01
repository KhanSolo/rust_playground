use std::thread;

fn main() {
    let handle = thread::spawn(||{
        println!("Hello, world!");
        42
    });
    let value = handle.join().unwrap();
    println!("{}", value);
}
