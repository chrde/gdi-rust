#![allow(dead_code)]
extern crate winapi;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate typed_builder;

use std::io;
use std::thread;

mod gui;

fn main() {
    match try_main() {
        Ok(code) => ::std::process::exit(code),
        Err(err) => {
            let msg = format!("Error: {}", err);
            panic!(msg);
        }
    }
}

fn try_main() -> io::Result<i32> {
    let handle = thread::spawn(move || {
        gui::new().unwrap();
    });
    handle.join().unwrap();
    Ok(0)
}