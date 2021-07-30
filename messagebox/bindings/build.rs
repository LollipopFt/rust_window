fn main() {
    windows::build!(Windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_OKCANCEL, MB_ICONERROR});
}