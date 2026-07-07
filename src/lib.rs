#![no_std]

mod vga;
mod panic;
mod mbi;
mod cpuid;
mod serial;
mod profile;
mod keyboard;
mod terminal;

use core::fmt::Write;

const MULTIBOOT_MAGIC: u32 = 0x2BADB002;

#[no_mangle]
pub extern "C" fn _start(magic: u32, mb_info_ptr: *const u8) -> ! {
    serial::SERIAL.init();

    let mut serial_w = serial::writer();
    let _ = writeln!(serial_w, "OmnOS v0.0.2 booting");

    let mut vga = vga::Writer::new();
    vga.clear();

    if magic != MULTIBOOT_MAGIC {
        let _ = writeln!(vga, "BAD MAGIC: {:#x}", magic);
        let _ = writeln!(serial_w, "BAD MAGIC: {:#x}", magic);
        loop {}
    }

    let memory_mib = mbi::parse(mb_info_ptr)
        .map(|m| m.mem_upper_mib())
        .unwrap_or(0);

    let sys = profile::SystemInfo::detect(memory_mib);
    let _ = sys.display(&mut vga);
    let _ = sys.display(&mut serial_w);

    if let Some(mbi) = mbi::parse(mb_info_ptr) {
        if let Some(name) = mbi.bootloader_name() {
            let _ = writeln!(vga, "  Bootloader: {}", name);
            let _ = writeln!(serial_w, "  Bootloader: {}", name);
        }
    }

    let _ = writeln!(vga);

    terminal::shell(&mut vga, mb_info_ptr)
}
