use std::io;

fn pisano_period(m: u128) -> u128 {
    let mut previous = 0;
    let mut current = 1;
    let mut period = 0u128;
    for i in 0..m * m {
        let tmp = (previous + current) % m;
        previous = current;
        current = tmp;
        if previous == 0 && current == 1 {
            period = i + 1;
            break;
        }
    }
    println!("pisano_period: {}", period);
    period
}

fn fibonacci_modulo(mut n: u128, m: u128) -> u128 {
    let period = pisano_period(m);
    n = n % period;
    let mut previous = 0;
    let mut current = 1;
    for _ in 0..n-1 {  
        println!("{} + {}", previous, current);      
        let tmp = previous + current;
        previous = current;
        current = tmp;
    }
    current % m
}

fn main() {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).unwrap();
    let split: Vec<&str> = guess.trim().split(" ").collect();
    let n: u128 = split[0].trim().parse().unwrap();
    let m: u128 = split[1].trim().parse().unwrap();

    let result = fibonacci_modulo(n, m);
    println!("{}", result);
}

/*
    (fibonacciModulo(1548276540 235)) #185
    (fibonacciModulo(11527523930876953 26673)) #10552
*/