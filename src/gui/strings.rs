use winapi::shared::ntdef::LPCWSTR;
use std::collections::HashMap;
use std::sync::Mutex;
use gui::utils::ToWide;
use gui::MAIN_WND_CLASS;
use gui::MAIN_WND_NAME;

lazy_static! {
    static ref HASHMAP: Mutex<HashMap<&'static str, Vec<u16>>> = {
    let mut m = HashMap::new();
    m.insert(MAIN_WND_CLASS, MAIN_WND_CLASS.to_wide_null());
    m.insert(MAIN_WND_NAME, MAIN_WND_NAME.to_wide_null());
        Mutex::new(m)
    };
}

pub fn get_string(str: &str) -> LPCWSTR {
    HASHMAP.lock().unwrap().get(str).unwrap().as_ptr() as LPCWSTR
}

pub fn set_string(str: &'static str, value: String) {
    HASHMAP.lock().unwrap().insert(str, value.to_wide_null());
}