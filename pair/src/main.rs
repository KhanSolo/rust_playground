use std::{num::ParseIntError, str::FromStr};

#[derive(Debug, PartialEq)]
struct Point {
    x:i32,
    y:i32,
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords : Vec<&str> // [&str;2]
         = s
        .trim_matches(|p| p=='(' || p==')')
        .split(",")
        .collect();

        let slice_of_str: &[&str] = &coords;
        println!("heap_var: {:p}", slice_of_str);
        println!("stack_var: {:p}", &coords);

        let x_coord= coords[0].parse::<i32>()?;
        let y_coord= coords[1].parse::<i32>()?;

        Ok(Point { x: x_coord, y: y_coord })
    }
}

fn main() {
    let parsed = Point::from_str("(1,2)").unwrap();
    println!("stack_var: {:p}", &parsed);

    let expected = Point{x:1,y:2};
    println!("stack_var: {:p}", &expected);

    assert_eq!(parsed, expected);
}
