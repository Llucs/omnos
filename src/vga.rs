use core::fmt;

const BUF: *mut u16 = 0xB8000 as *mut u16;
const W: usize = 80;
const H: usize = 25;

pub struct Writer {
    col: usize,
    row: usize,
}

impl Writer {
    pub fn new() -> Self {
        Writer { col: 0, row: 0 }
    }

    fn put(&mut self, b: u8) {
        if self.row >= H {
            return;
        }
        let pos = self.row * W + self.col;
        unsafe {
            BUF.add(pos).write_volatile(b as u16 | (0x0F << 8));
        }
        self.col += 1;
        if self.col >= W {
            self.col = 0;
            self.row += 1;
        }
    }

    pub fn clear(&self) {
        for i in 0..(W * H) {
            unsafe {
                BUF.add(i).write_volatile(0x0F20);
            }
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for &b in s.as_bytes() {
            match b {
                b'\n' => {
                    self.col = 0;
                    self.row += 1;
                }
                _ => self.put(b),
            }
        }
        Ok(())
    }
}
