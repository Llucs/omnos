use core::fmt;
use core::ptr;

const PORT: usize = 0x3F8;

pub struct SerialPort;

impl SerialPort {
    pub const fn new() -> Self {
        SerialPort
    }

    pub fn init(&self) {
        unsafe {
            ptr::write_volatile((PORT + 1) as *mut u8, 0x00);
            ptr::write_volatile((PORT + 3) as *mut u8, 0x80);
            ptr::write_volatile(PORT as *mut u8, 0x0C);
            ptr::write_volatile((PORT + 1) as *mut u8, 0x00);
            ptr::write_volatile((PORT + 3) as *mut u8, 0x03);
        }
    }

    pub fn write_byte(&self, b: u8) {
        unsafe {
            while (ptr::read_volatile((PORT + 5) as *const u8) & 0x20) == 0 {}
            ptr::write_volatile(PORT as *mut u8, b);
        }
    }

    pub fn write_str(&self, s: &str) {
        for &b in s.as_bytes() {
            self.write_byte(b);
        }
    }
}

pub struct Writer;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for &b in s.as_bytes() {
            unsafe {
                while (ptr::read_volatile((PORT + 5) as *const u8) & 0x20) == 0 {}
                ptr::write_volatile(PORT as *mut u8, b);
            }
        }
        Ok(())
    }
}

pub fn writer() -> Writer {
    Writer
}

pub static SERIAL: SerialPort = SerialPort::new();
