use bindings::Windows::Win32::{
    Foundation::{HINSTANCE, HWND, WPARAM, LPARAM, LRESULT},
    UI::WindowsAndMessaging::{WNDCLASSW, WNDCLASS_STYLES, CS_OWNDC, CS_HREDRAW, CS_VREDRAW, DefWindowProcW, HICON}
};

fn main() {
    let expr = true;
    let style: WNDCLASS_STYLES = {CS_OWNDC; CS_HREDRAW; CS_VREDRAW};
    let lpfnWndProc = Some(DefWindowProcW);
    let cbClsExtra = 0;
    let cbWndExtra = 0;
    let hInstance = HINSTANCE;
    let hIcon = HICON;
    let hCursor = expr;
    let hbrBackground = expr;
    let lpszMenuName = expr;
    let lpszClassName = expr;

    let mut winclass = WNDCLASSW { style,
                                   lpfnWndProc,
                                   cbClsExtra,
                                   cbWndExtra,
                                   hInstance,
                                   hIcon,
                                   hCursor,
                                   hbrBackground,
                                   lpszMenuName,
                                   lpszClassName };
}