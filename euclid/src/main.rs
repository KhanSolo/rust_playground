use std::io;

fn main() {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).unwrap();
    let split: Vec<&str> = guess.trim().split(" ").collect();
    let mut a: u32 = split[0].trim().parse().unwrap();
    let mut b: u32 = split[1].trim().parse().unwrap();

    while a != 0 && b != 0 {
        if a > b {
            a = a % b;
        } else {
            b = b % a;
        }
    }
    println!("{}", a + b);
}
