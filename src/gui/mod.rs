use winapi::um::winuser::IsGUIThread;
use winapi::shared::minwindef::TRUE;
use gui::strings::get_string;
use std::io;
use winapi::shared::minwindef::UINT;
use winapi::shared::minwindef::WPARAM;
use winapi::shared::windef::HWND;
use winapi::shared::minwindef::LPARAM;
use winapi::shared::minwindef::LRESULT;
use winapi::um::winuser::WS_OVERLAPPEDWINDOW;
use gui::event::Event;
use winapi::um::winuser::WM_CREATE;
use winapi::um::winuser::WM_QUIT;
use gui::gui::Gui;
use gui::msg::Msg;
use winapi::um::winuser::SetWindowLongPtrW;
use winapi::um::winuser::GWLP_USERDATA;
use winapi::shared::basetsd::LONG_PTR;
use winapi::um::winuser::GetWindowLongPtrW;
use winapi::um::winuser::SW_SHOWDEFAULT;
use winapi::um::winuser::MSG;

const MAIN_WND_CLASS: &str = "my_class";
const MAIN_WND_NAME: &str = "my main window";

mod wnd_class;
mod wnd;
mod utils;
mod strings;
mod gui;
mod msg;
mod event;

pub fn new() -> io::Result<i32> {
    let res = unsafe { IsGUIThread(TRUE) };
    assert_ne!(res, 0);
    let class = wnd_class::WndClass::new(get_string(MAIN_WND_CLASS), wnd_proc)?;
    let params = wnd::WndParams::builder()
        .window_name(get_string(MAIN_WND_NAME))
        .class_name(class.0)
        .instance(class.1)
        .style(WS_OVERLAPPEDWINDOW)
        .build();
    let wnd = wnd::Wnd::new(params)?;
    wnd.show(SW_SHOWDEFAULT);
    wnd.update()?;
    loop {
        match MSG::get(None).unwrap() {
            MSG { message: WM_QUIT, wParam: code, .. } => {
                return Ok(code as i32);
            }
            msg => {
                msg.translate();
                msg.dispatch();
            }
        }
    }
}

pub unsafe extern "system" fn wnd_proc(wnd: HWND, message: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if message == WM_CREATE {
        let gui = Box::new(Gui::new());
        SetWindowLongPtrW(wnd, GWLP_USERDATA, Box::into_raw(gui) as LONG_PTR);
        0
    } else {
        let event = Event::new(wnd, message, l_param, w_param);
        let gui = &*(GetWindowLongPtrW(wnd, GWLP_USERDATA) as *const Gui);
        gui.handle(event)
    }
}
