use std::io;

fn fibonacci_modulo2(n: u128, m: u128) -> u128 {
    let mut fib_prev = 0u128;
    let mut fib = 1u128;
    let mut cached = vec![fib_prev, fib];

    for _curr in 1..n {
        let fib_old = fib;
        fib = (fib + fib_prev) % m;
        fib_prev = fib_old;

        if fib_prev == 0 && fib == 1 {
            cached.pop();
            break;
        } else {
            cached.push(fib);
        }
    }
    let offset = n % (cached.len() as u128);
    cached[offset as usize]
}

fn main() {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).unwrap();
    let split: Vec<&str> = guess.trim().split(" ").collect();
    let n: u128 = split[0].trim().parse().unwrap();
    let m: u128 = split[1].trim().parse().unwrap();

    let result = fibonacci_modulo2(n, m);
    println!("{}", result);
}

/*
    (fibonacciModulo(1548276540 235)) #185
    (fibonacciModulo(11527523930876953 26673)) #10552
*/
