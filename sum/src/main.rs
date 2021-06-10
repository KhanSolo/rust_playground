use std::io;
fn main() {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Не удалось прочитать строку");
    let split = guess.trim().split(" ");
    let mut sum = 0;
    for a in split {
        let b : i32 = a.parse().expect("Не удалось распарсить строку");
        sum = sum + b;
    }
    println!("{}",sum);
}