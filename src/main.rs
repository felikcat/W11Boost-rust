pub mod resource;
pub mod macros;

use macros::make_int_resource;
use resource::*;
use std::mem;
use std::os::raw::c_void;
use std::process::exit;
use std::ptr::null_mut;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::System::Environment::GetCommandLineW;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::WindowsProgramming::MulDiv;
use windows::Win32::UI::HiDpi::{GetDpiForSystem, GetDpiForWindow};
use windows::{
    Win32::Foundation::*, Win32::Graphics::Gdi::ValidateRect, Win32::UI::WindowsAndMessaging::*,
    core::*,
};

struct Button {
    width: i32,
    height: i32,
    apply: HWND,
}

struct Common {
    left: i32,
    right: i32,
    top: i32,
    bottom: i32,
    center_width: i32,
    center_height: i32,
}

fn main() -> Result<()> {
    let instance = unsafe { GetModuleHandleW(None).unwrap() }.into();
    let cmd_line = unsafe { GetCommandLineW() };
    let cmd_show = { SW_SHOW };

    let main_result = winmain(instance, None, cmd_line, cmd_show);
    if main_result.is_err() {
        println!("Something I didn't account for errored");
    }
    Ok(())
}

fn winmain(
    instance: HINSTANCE,
    _prev_instance: Option<HINSTANCE>,
    _cmd_line: PCWSTR,
    cmd_show: SHOW_WINDOW_CMD,
) -> Result<()> {
    unsafe {
        let window_class = w!("W11BOOST");
        let resource_id = make_int_resource(IDI_W11BOOST);
        let mut message = MSG::default();

        let wcex = WNDCLASSW {
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wndproc),
            hInstance: instance.into(),
            hCursor: LoadCursorW(None, IDC_ARROW)?,
            hbrBackground: HBRUSH((COLOR_WINDOW.0 + 1) as *mut c_void),
            lpszClassName: window_class,
            lpszMenuName: PCWSTR(resource_id),
            ..Default::default()
        };

        let atom = RegisterClassW(&wcex);
        debug_assert!(atom != 0);

        let dpi = GetDpiForSystem();
        let width = MulDiv(640, dpi as i32, 100);
        let height = MulDiv(480, dpi as i32, 100);

        let hwnd = CreateWindowExW(
            WINDOW_EX_STYLE::default(),
            window_class,
            w!("W11Boost"),
            WS_OVERLAPPED | WS_MINIMIZEBOX | WS_SYSMENU,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            width,
            height,
            None,
            None,
            instance,
            None,
        )?;

        let monitor = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);
        let mut mi = MONITORINFO {
            cbSize: size_of::<MONITORINFO>() as u32,
            rcMonitor: RECT::default(),
            rcWork: RECT::default(),
            dwFlags: 0,
        };

        if GetMonitorInfoW(monitor, &mut mi).into() {
            let rc_work = mi.rcWork;
            let x = rc_work.left + (rc_work.right - rc_work.left - width) / 2;
            let y = rc_work.top + (rc_work.bottom - rc_work.top - height) / 2;

            println!("[{},{},{},{}]", x, y, width, height);

            SetWindowPos(hwnd, None, x, y, 0, 0, SWP_NOSIZE | SWP_NOZORDER)?;
        }

        let show_window = ShowWindow(hwnd, cmd_show);
        if (show_window).into() {
            MessageBoxW(hwnd, w!("ShowWindow failed"), w!("W11Boost"), MB_OK);
            exit(1);
        }
        let update_window = UpdateWindow(hwnd);
        if (!update_window).into() {
            MessageBoxW(hwnd, w!("UpdateWindow failed"), w!("W11Boost"), MB_OK);
            exit(1);
        }
        
        // Main message loop:
        while GetMessageW(&mut message, None, 0, 0).into() {
            _ = TranslateMessage(&message); // Do not check for errors on this ever
            DispatchMessageW(&message);
        }
    }
    Ok(())
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_CREATE => {
                println!("WM_CREATE");

                let font_size = 24;
                let font_dpi = USER_DEFAULT_SCREEN_DPI;
                let window_dpi = GetDpiForWindow(window);
                let scale_font = MulDiv(font_size, font_dpi as i32, window_dpi as i32);

                let font = CreateFontW(
                    scale_font,
                    0,
                    0,
                    0,
                    FW_LIGHT.0 as i32,
                    0,
                    0,
                    0,
                    ANSI_CHARSET.0 as u32,
                    OUT_OUTLINE_PRECIS.0 as u32,
                    CLIP_DEFAULT_PRECIS.0 as u32,
                    CLEARTYPE_NATURAL_QUALITY,
                    DEFAULT_PITCH.0 as u32 | FF_DONTCARE.0 as u32,
                    w!("Segoe UI"),
                );

                let mut rc_client = RECT::default();
                let gcr = GetClientRect(window, &mut rc_client);
                if gcr.is_err() {
                    MessageBoxW(window, w!("GetClientRect failed"), w!("W11Boost"), MB_OK);
                    exit(1);
                }

                let common = Common {
                    left: rc_client.left + 4,
                    right: rc_client.right - 8,
                    top: rc_client.top,
                    bottom: rc_client.bottom - 4,
                    center_width: rc_client.right / 2,
                    center_height: rc_client.bottom / 2,
                };

                let mut button = Button {
                    width: common.right,
                    height: (common.center_height * 2) / 10,
                    apply: HWND(null_mut()),
                };

                let button_apply_hmenu: HMENU = mem::transmute(IDC_APPLY_W11BOOST);

                button.apply = CreateWindowExW(
                    WINDOW_EX_STYLE::default(),
                    w!("BUTTON"),
                    w!("Apply W11Boost"),
                    WS_CHILD | WS_VISIBLE | WINDOW_STYLE(BS_PUSHBUTTON as u32).try_into().unwrap() | WINDOW_STYLE(BS_FLAT as u32).try_into().unwrap(),
                    common.left,
                    common.bottom,
                    button.width,
                    button.height,
                    window,
                    button_apply_hmenu,
                    None,
                    None,
                )
                .expect("Failed to create: button.apply");

                let buttons = [button.apply];
                let font_cast: WPARAM = mem::transmute(font);
                let lparam_value: LPARAM = LPARAM(1); // TRUE

                for &window in buttons.iter() {
                    SendMessageW(window, WM_SETFONT, font_cast, lparam_value);
                }

                LRESULT(0)
            }
            WM_COMMAND => {
                println!("WM_COMMAND");
                LRESULT(0)
            }
            WM_PAINT => {
                println!("WM_PAINT");
                let mut ps: PAINTSTRUCT = Default::default();
                let hdc: HDC = BeginPaint(window, &mut ps);

                if hdc.is_invalid() {
                    MessageBoxW(window, w!("BeginPaint failed"), w!("W11Boost"), MB_OK);
                    exit(1);
                }

                let color_menu_index = COLOR_MENU.0 + 1;
                let color_menu_cast: HBRUSH = mem::transmute(&color_menu_index);
                FillRect(hdc, &ps.rcPaint, color_menu_cast);

                let end_paint = EndPaint(window, &ps);
                if (!end_paint).into() {
                    MessageBoxW(window, w!("EndPaint failed"), w!("W11Boost"), MB_OK);
                    exit(1);
                }

                _ = ValidateRect(window, None);
                LRESULT(0)
            }
            WM_DESTROY => {
                println!("WM_DESTROY");
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcW(window, message, wparam, lparam),
        }
    }
}
