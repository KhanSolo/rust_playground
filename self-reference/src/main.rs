use std::fs::read_to_string;

fn main() {
    let input = match read_to_string("lines.txt") {
        Ok(read) => read,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    let good_lines: Vec<_> = input
        .lines()
        .filter(|l| starts_with_capital_letter(l))
        .collect();

    dbg!(good_lines);
}

fn starts_with_capital_letter(s: &str) -> bool {
    matches!(
        s.chars().next(), // what
        Some(c) if c.is_uppercase() // condition
    )
}
