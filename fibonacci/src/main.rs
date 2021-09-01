fn main() {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let num: u32 = line.trim().parse().unwrap();

    if num < 3 {
        println!("{}", 1);
        return;
    }

    let mut f0:u32 = 0;
    let mut f1:u32 = 1;
    let mut cur:u32 = 0;

    for _ in 2..=num {        
        cur = f0 + f1;
        f0 = f1;
        f1 = cur;
    }

    println!("{}", cur);
}
