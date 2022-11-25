use windows_sys::{core::*, 
    Win32::UI::WindowsAndMessaging::*};

fn main() {
    unsafe {
        MessageBoxW(0, w!("Ansi"), w!("World"), MB_OK);
    }
}