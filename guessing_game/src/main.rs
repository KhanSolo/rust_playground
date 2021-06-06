use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    let range = 1u32..101u32;
    let secret_number = rand::thread_rng().gen_range(range);
    //println!("Секретное число равно {}", secret_number);

    loop {

        println!("Введите число");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Не удалось прочитать строку");
        let guess : u32 = match guess.trim().parse() {        
            Ok(num) => num,
            Err(e) => {
                println!("{}",e);
                continue;
            }
        };

        println!("Вы загадали {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Слишком маленькое"),
            Ordering::Greater => println!("Слишком большое"),
            Ordering::Equal => {
                println!("Вы выиграли");
                break;
            },
        }
    }
}
