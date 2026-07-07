#![no_std]

mod vga;
mod panic;

use core::fmt::Write;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut w = vga::Writer::new();
    w.clear();
    writeln!(w, "OmnOS v0.0.1").unwrap();
    writeln!(w, "Adaptive kernel :: nano / lite / desktop / scale").unwrap();
    loop {}
}
