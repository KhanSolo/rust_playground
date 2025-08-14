use std::io::{self, Write};
use std::ffi::CString;

// Declaring wrapping trait
trait AsBytes {
    fn as_bytes_ref(&self) -> &[u8];
}

// Path is platform specific! 
// Windows uses utf-16   Cow::Borrowed(self.as_os_str().as_bytes())
// Unix   Cow::Borrowed(self.as_os_str().as_bytes())
// impl AsBytes for Path { 
//     fn as_bytes_ref(&self) -> &[u8] {
//         match self.to_string_lossy() {
//             Cow::Borrowed(s) => s.as_bytes(),
//             Cow::Owned(s) => s.into_bytes(),
//         }
//     }
// }

impl AsBytes for CString {
    fn as_bytes_ref(&self) -> &[u8] { self.as_bytes() }
}
impl AsBytes for String {
    fn as_bytes_ref(&self) -> &[u8] { self.as_bytes() }
}
impl AsBytes for &str {
    fn as_bytes_ref(&self) -> &[u8] { self.as_bytes() }
}

fn info<T:AsBytes>(text: T) {
    io::stdout().write_all(text.as_bytes_ref()).unwrap();
    io::stdout().write_all(b"\n").unwrap();
}

fn main() {
    let string_slice = "Hello, &str!";
    info(string_slice);  // &str → &[u8]

    let owned_string = String::from("Hello, String!");    
    info(owned_string);  // String → &[u8]

    info("Also works with string literals!");  // &'static str → &[u8]

    let cstring = CString::new("Hello, CString!").unwrap();
    info(cstring);  // CString → &[u8]

    // let path = Path::new("/tmp/rust");
    // info(path);
}