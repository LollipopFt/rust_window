fn main() {
    windows::build!(
        Windows::Win32::{
            Foundation::HINSTANCE,
            UI::WindowsAndMessaging::{WNDCLASSW, WNDCLASS_STYLES, DefWindowProcW}
        }
    );
}
