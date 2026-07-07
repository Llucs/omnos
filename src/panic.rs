use core::panic::PanicInfo;
use core::fmt::Write;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut vga = crate::vga::Writer::new();
    let _ = write!(vga, "OmnOS PANIC: {}", info);
    let mut serial = crate::serial::writer();
    let _ = write!(serial, "OmnOS PANIC: {}", info);
    loop {}
}
