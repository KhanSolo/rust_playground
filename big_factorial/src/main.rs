use num_bigint::BigUint;

fn main() {
    let factorial = |x: usize| -> BigUint { (1..=x).map(BigUint::from).product()};
    let result = factorial(1000);
    println!("{}", result);
}
