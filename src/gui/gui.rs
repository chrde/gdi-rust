use gui::event::Event;
use winapi::shared::minwindef::LRESULT;
use winapi::um::winuser::DefWindowProcW;

pub struct Gui {}

impl Gui {
    pub fn new() -> Gui {
        Gui {}
    }

    pub fn handle(&self, event: Event) -> LRESULT {
        unsafe { DefWindowProcW(event.wnd(), event.message(), event.w_param(), event.l_param()) }
    }
}