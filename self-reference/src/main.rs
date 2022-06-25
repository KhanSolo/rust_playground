use std::fs::read_to_string;

fn main() {
    let input = match read_to_string("lines.txt") {
        Ok(read) => read,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    for str in input.lines() {
        println!("{}", str);
    }
}
