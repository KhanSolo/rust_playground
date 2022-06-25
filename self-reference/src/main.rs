use std::fs::read_to_string;

#[derive(Debug)]
struct S<'a> {
    input: String,
    good_lines: Vec<&'a str>,
}

fn main() {
    let input = match read_to_string("lines.txt") {
        Ok(read) => read,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    let mut s = S {
        input,
        good_lines: vec![],
    };

    s.good_lines = s
        .input
        .lines()
        .filter(|l| starts_with_capital_letter(l))
        .collect();

    for line in &s.good_lines {
        println!("{}", line);
    }

    dbg!(&s);
}

fn starts_with_capital_letter(s: &str) -> bool {
    matches!(
        s.chars().next(), // what
        Some(c) if c.is_uppercase() // condition
    )
}
