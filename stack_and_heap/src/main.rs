#[derive(Debug)]
struct MyStruct{
    value: i32,
}

impl MyStruct {
    fn new(value: i32) -> Self{
        Self{
            value
        }
    }
}

fn main() {
    let a = MyStruct::new(17);
    let b = Box::new(a);
    dbg!(*b);
}
