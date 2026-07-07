#![no_std]

mod vga;
mod panic;
mod mbi;
mod cpuid;
mod serial;
mod profile;

use core::fmt::Write;

const MULTIBOOT_MAGIC: u32 = 0x2BADB002;

#[no_mangle]
pub extern "C" fn _start(magic: u32, mb_info_ptr: *const u8) -> ! {
    serial::SERIAL.init();

    let mut serial = serial::writer();
    let _ = writeln!(serial, "OmnOS v0.0.1 booting");

    let mut vga = vga::Writer::new();
    vga.clear();

    if magic != MULTIBOOT_MAGIC {
        let _ = writeln!(vga, "BAD MAGIC: {:#x}", magic);
        let _ = writeln!(serial, "BAD MAGIC: {:#x}", magic);
        loop {}
    }

    let memory_mib = mbi::parse(mb_info_ptr)
        .map(|m| m.mem_upper_mib())
        .unwrap_or(0);

    let sys = profile::SystemInfo::detect(memory_mib);
    let _ = sys.display(&mut vga);
    let _ = sys.display(&mut serial);

    if let Some(mbi) = mbi::parse(mb_info_ptr) {
        if let Some(name) = mbi.bootloader_name() {
            let _ = writeln!(vga, "  Bootloader: {}", name);
            let _ = writeln!(serial, "  Bootloader: {}", name);
        }
        let usable = mbi.memory_map_entries().filter(|e| e.is_usable()).count();
        let _ = writeln!(vga, "  Usable regions: {}", usable);
        let _ = writeln!(serial, "  Usable regions: {}", usable);
    }

    let _ = writeln!(vga);
    let _ = writeln!(vga, "OmnOS does not have minimum requirements.");
    let _ = writeln!(vga, "It has adaptive modes.");
    let _ = writeln!(serial, "Boot complete. Halting.");

    loop {}
}
