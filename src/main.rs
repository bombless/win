use windows::Win32::UI::WindowsAndMessaging::{
    WNDCLASSA, WS_OVERLAPPEDWINDOW, CW_USEDEFAULT, MSG, WM_DESTROY, WM_PAINT,
    WINDOW_EX_STYLE, SHOW_WINDOW_CMD, HMENU, GWLP_HINSTANCE, BS_DEFPUSHBUTTON,
    WS_TABSTOP, WS_VISIBLE, WS_CHILD, WINDOW_STYLE,
    CreateWindowExA, RegisterClassA, ShowWindow, GetMessageA, TranslateMessage,
    DispatchMessageA, PostQuitMessage, DefWindowProcA, GetWindowLongPtrA
};
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::Foundation::{LRESULT, LPARAM, WPARAM, HWND, HINSTANCE};
use windows::core::PCSTR;
use windows::Win32::Graphics::Gdi::{
    PAINTSTRUCT, BeginPaint, FillRect, EndPaint, COLOR_WINDOW, HBRUSH
};
fn main() {
    
    unsafe {
        let h_instance = GetModuleHandleA(PCSTR(std::ptr::null())).unwrap();
        println!("h_instance {h_instance:?}");
        let class_name: PCSTR = PCSTR("test\0\0".as_ptr() as _);
        let wc = WNDCLASSA {
            lpfnWndProc: Some(wnd_proc),
            lpszClassName: class_name.clone(),
            hInstance: h_instance.into(),
            ..Default::default()
        };
        println!("wc {wc:?}");
    
        let window_title = PCSTR("nothing\0\0".as_ptr() as _);
        let atom = RegisterClassA(&wc as _);
        if atom == 0 {
            panic!("RegisterClassA failed");
        }
        let hwnd = CreateWindowExA(WINDOW_EX_STYLE(0), class_name, window_title, WS_OVERLAPPEDWINDOW,
           CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT, CW_USEDEFAULT,
           HWND(0), HMENU(0), h_instance, None
        );

        println!("hwnd {hwnd:?}");

        if hwnd == HWND(0) {
            panic!("CreateWindowExA failed");
        }
        ShowWindow(hwnd, SHOW_WINDOW_CMD(1));

        create_window(hwnd, Class::Button, PCSTR("OK\0".as_ptr() as _), 10, 10, 100, 100);

        let mut msg = MSG::default();

        while GetMessageA(&mut msg as _, HWND(0), 0, 0).0 > 0 {
            TranslateMessage(&mut msg as _);
            DispatchMessageA(&mut msg as _);
        }
    }
}
enum Class {
    Button,
    Static,
}
impl Into<PCSTR> for Class {
    fn into(self) -> PCSTR {
        match self {
            Class::Button => PCSTR("BUTTON\0".as_ptr() as _),
            Class::Static => PCSTR("STATIC\0".as_ptr() as _),
        }
    }
}
unsafe fn create_window(parent: HWND, class: Class, text: PCSTR, x: i32, y: i32, w: i32, h: i32) -> HWND {
    let class: PCSTR = class.into();
    CreateWindowExA(
        WINDOW_EX_STYLE(0),
        class,  // Predefined class; Unicode assumed 
        text,      // Button text 
        WS_TABSTOP | WS_VISIBLE | WS_CHILD | WINDOW_STYLE(BS_DEFPUSHBUTTON as _),  // Styles 
        x,         // x position 
        y,         // y position 
        w,        // Button width
        h,        // Button height
        parent,     // Parent window
        HMENU(0),       // No menu.
        HINSTANCE(GetWindowLongPtrA(parent, GWLP_HINSTANCE) as _),
        None)
}
unsafe extern "system" fn wnd_proc(hwnd: HWND, param1: u32, param2: WPARAM, param3: LPARAM) -> LRESULT {
    match param1 {
        WM_DESTROY => { PostQuitMessage(0); },
        WM_PAINT => {
            let mut ps = PAINTSTRUCT::default();
            let hdc = BeginPaint(hwnd, &mut ps as _);
            FillRect(hdc, &ps.rcPaint as _, HBRUSH((COLOR_WINDOW.0 + 1) as _));
            EndPaint(hwnd, &ps as _);
        },
        _ => { return DefWindowProcA(hwnd, param1, param2, param3); }
    }
    LRESULT(0)
}
