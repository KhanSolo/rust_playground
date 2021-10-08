use std::fmt::{Arguments, Formatter};

struct Door {
    is_open: bool,
}

impl Door {
    fn new(is_open: bool) -> Door {
        Door { is_open }
    }
}

trait Openable {
    fn open(&mut self);
}

impl Openable for Door {
    fn open(&mut self) {
        self.is_open = true;
    }
}

impl ::core::fmt::Debug for Door {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { 
        f.write_str("data")
        //f.write_fmt(Arguments{})
     }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_door() {
        let mut door = Door::new(false);
        door.open();
        assert!(door.is_open);
    }
}

fn main() {
    let door = Door::new(false);
    println!("door: `{:?}`", door);
}