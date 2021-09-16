use std::io;

fn pisano_period(m: u64) -> u64 {
    let mut previous = 0;
    let mut current = 1;
    let mut period = 0u64;
    for i in 0..=m * m {
        let tmp1 = current;
        let tmp2 = (previous + current) % m;
        previous = tmp1;
        current = tmp2;
        if previous == 0 && current == 1 {
            period = i + 1;
            break;
        }
    }
    //println!("pisano_period: {}", period);
    period
}

fn fibonacci_modulo(mut n: u64, m: u64) -> u64 {
    let period = pisano_period(m);
    n = n % period;
    let mut previous = 0;
    let mut current = 1;
    for _ in 0..n-1 {
        let tmp1 = current;
        let tmp2 = previous + current;
        previous = tmp1;
        current = tmp2;
    }
    current % m
}

fn main() {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).unwrap();
    let split: Vec<&str> = guess.trim().split(" ").collect();
    let n: u64 = split[0].trim().parse().unwrap();
    let m: u64 = split[1].trim().parse().unwrap();

    let result = fibonacci_modulo(n, m);
    println!("{}", result);
}
