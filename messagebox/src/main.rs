#![allow(path_statements)]

use bindings::Windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_OKCANCEL, MB_ICONERROR};

fn main() {
    unsafe {
        MessageBoxA(None, "Error Box", "Encountered Error 404", {MB_OKCANCEL; MB_ICONERROR});
    }
}