use core::panic::PanicInfo;
use core::fmt::Write;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut w = crate::vga::Writer::new();
    let _ = write!(w, "OmnOS PANIC: {}", info);
    loop {}
}
