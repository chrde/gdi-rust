use winapi::shared::minwindef::LPARAM;
use winapi::shared::minwindef::WPARAM;
use winapi::shared::minwindef::UINT;
use winapi::shared::windef::HWND;

#[derive(Copy, Clone)]
pub struct Event {
    wnd: HWND,
    l_param: LPARAM,
    w_param: WPARAM,
    message: UINT,
}

impl Event {
    pub fn new(wnd: HWND, message: UINT, l_param: LPARAM, w_param: WPARAM) -> Event {
        Event { wnd, message, l_param, w_param }
    }

    pub fn wnd(&self) -> HWND {
        self.wnd
    }

    pub fn message(&self) -> UINT {
        self.message
    }

    pub fn l_param(&self) -> LPARAM {
        self.l_param
    }

    pub fn w_param(&self) -> WPARAM {
        self.w_param
    }

    pub fn l_param_mut<T>(&self) -> *mut T {
        self.l_param as *mut T
    }

    pub fn w_param_mut<T>(&self) -> *mut T {
        self.w_param as *mut T
    }
}
