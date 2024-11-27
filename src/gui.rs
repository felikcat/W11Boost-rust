use fltk::{
    app,
    button::{Button, CheckButton},
    enums::{self, Color},
    frame,
    prelude::*,
    window::{OverlayWindow, Window},
};
use fltk_theme::{ColorTheme, WidgetScheme, WidgetTheme, color_themes, widget_themes};
use std::{
    error::Error,
    ffi::c_void,
    mem,
    ptr::{self, null, null_mut},
};
use windows::Win32::{
    Foundation::{HWND, RECT},
    UI::
        WindowsAndMessaging::{GetWindowLongPtrW, GetWindowRect, SetWindowLongPtrW, SetWindowPos, GWL_EXSTYLE, GWL_STYLE, HWND_TOPMOST, SWP_FRAMECHANGED, SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW, WS_EX_APPWINDOW, WS_SYSMENU}
    ,
};

type MyCheckboxes = [CheckButton; 6];

pub fn draw_gui() -> Result<(), Box<dyn Error>> {
    let app = app::App::default().with_scheme(app::Scheme::Gtk);
    let font = app.load_font("C:\\Windows\\Fonts\\segoeui.ttf").unwrap();

    let mut wind = Window::default()
        .with_label("W11Boost")
        .with_size(480, 360)
        .center_screen();
    // wind.set_border(false);

    let widget_theme = ColorTheme::new(color_themes::BLACK_THEME);
    widget_theme.apply();

    let mut titlebar = frame::Frame::new(0, 0, 480, 32, None);
    titlebar.set_frame(enums::FrameType::FlatBox);
    titlebar.set_color(Color::from_rgb(22, 22, 22));

    let mut titlebar_close =
        Button::new(wind.width() - 32, 0, 32, 32, "X").set_frame(enums::FrameType::NoBox);

    let mut apply = Button::new(
        0,
        0,
        wind.width() - 2,
        wind.height() / 10 * 2,
        "Apply W11Boost",
    )
    .center_of(&wind);

    apply.set_pos(
        wind.width() - apply.width() - 1,
        wind.height() - apply.height() - 2,
    ); // Put button at the bottom

    apply.set_label_font(enums::Font::by_name(&font));
    apply.set_label_size(16);

    let checkbox_height = wind.height() / 10;

    let mut my_checkboxes: MyCheckboxes = [
        CheckButton::new(
            0,
            titlebar.height(),
            wind.width(),
            checkbox_height,
            "Reduce local data collection",
        ),
        CheckButton::new(
            0,
            titlebar.height() + wind.height() / 10 + 2,
            wind.width(),
            checkbox_height,
            "Reduce online data collection",
        ),
        CheckButton::new(
            0,
            titlebar.height() + wind.height() / 10 * 2 + 4,
            wind.width(),
            checkbox_height,
            "Create a system restore point",
        ),
        CheckButton::new(
            0,
            titlebar.height() + wind.height() / 10 * 3 + 6,
            wind.width(),
            checkbox_height,
            "Install the Microsoft Store",
        ),
        CheckButton::new(
            0,
            titlebar.height() + wind.height() / 10 * 4 + 8,
            wind.width(),
            checkbox_height,
            "Install support for .appx/.appxbundle and WinGet",
        ),
        CheckButton::new(
            0,
            titlebar.height() + wind.height() / 10 * 5 + 10,
            wind.width(),
            checkbox_height,
            "Disable sleep and hibernate",
        ),
    ];

    for checkbox in &mut my_checkboxes[0..6] {
        checkbox.set_label_font(enums::Font::by_name(&font));
        checkbox.set_label_size(16);
    }

    my_checkboxes[2].set_value(true);

    wind.end();
    wind.show();

    let hwnd = wind.raw_handle();
    let hwnd: HWND = unsafe { mem::transmute(hwnd) };

    const WS_SYSMENU: isize = 0x80000;
    const WS_EX_APPWINDOW: isize = 0x40000;
    const WS_CAPTION: isize = 0xC0000;

    unsafe {
        //let old_style = GetWindowLongPtrW(hwnd, GWL_STYLE);
        let new_style = WS_SYSMENU | WS_EX_APPWINDOW | WS_CAPTION;
        let mut size_rect: RECT = Default::default();

        GetWindowRect(hwnd, &mut size_rect)?;
        SetWindowLongPtrW(hwnd, GWL_STYLE,  new_style);

        SetWindowPos(hwnd, HWND_TOPMOST, 0, 0, 0, 0, SWP_SHOWWINDOW | SWP_NOMOVE | SWP_NOSIZE)?;
        SetWindowPos(hwnd, None, size_rect.left, size_rect.top, size_rect.right - size_rect.left, size_rect.bottom - size_rect.top, SWP_FRAMECHANGED | SWP_NOMOVE | SWP_NOSIZE)?;
    }

    app.run().unwrap();
    Ok(())
}
