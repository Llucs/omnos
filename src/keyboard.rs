use core::ptr;

const DATA: u16 = 0x60;
const STATUS: u16 = 0x64;

fn read_data() -> u8 {
    unsafe { ptr::read_volatile(DATA as *const u8) }
}

fn status() -> u8 {
    unsafe { ptr::read_volatile(STATUS as *const u8) }
}

fn wait_key() {
    while status() & 1 == 0 {}
}

fn next_scancode() -> u8 {
    wait_key();
    read_data()
}

const KEY_MAP: [u8; 137] = [
    0, 0, b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'0', b'-', b'=', 0, 0,
    b'q', b'w', b'e', b'r', b't', b'y', b'u', b'i', b'o', b'p', b'[', b']', 0, 0, b'a', b's',
    b'd', b'f', b'g', b'h', b'j', b'k', b'l', b';', b'\'', b'`', 0, b'\\', b'z', b'x', b'c',
    b'v', b'b', b'n', b'm', b',', b'.', b'/', 0, b'*', 0, b' ', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, b'7', b'8', b'9', b'-', b'4', b'5', b'6', b'+', b'1', b'2', b'3',
    b'0', b'.', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

const KEY_MAP_SHIFT: [u8; 137] = [
    0, 0, b'!', b'@', b'#', b'$', b'%', b'^', b'&', b'*', b'(', b')', b'_', b'+', 0, 0,
    b'Q', b'W', b'E', b'R', b'T', b'Y', b'U', b'I', b'O', b'P', b'{', b'}', 0, 0, b'A', b'S',
    b'D', b'F', b'G', b'H', b'J', b'K', b'L', b':', b'"', b'~', 0, b'|', b'Z', b'X', b'C',
    b'V', b'B', b'N', b'M', b'<', b'>', b'?', 0, b'*', 0, b' ', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, b'7', b'8', b'9', b'-', b'4', b'5', b'6', b'+', b'1', b'2', b'3',
    b'0', b'.', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

pub struct Key {
    pub ascii: Option<u8>,
    pub enter: bool,
    pub backspace: bool,
}

pub struct Keyboard;

impl Keyboard {
    pub fn new() -> Self {
        Keyboard
    }

    pub fn read_key(&self) -> Key {
        loop {
            let sc = next_scancode();
            if sc & 0x80 != 0 {
                continue;
            }
            let idx = sc as usize;
            if idx == 0x2A || idx == 0x36 {
                continue;
            }
            if idx == 0x1C {
                return Key { ascii: None, enter: true, backspace: false };
            }
            if idx == 0x0E {
                return Key { ascii: None, enter: false, backspace: true };
            }
            if idx < 128 {
                let c = KEY_MAP[idx];
                if c != 0 {
                    return Key { ascii: Some(c), enter: false, backspace: false };
                }
            }
        }
    }

    pub fn read_line(&self, buf: &mut [u8]) -> usize {
        let mut pos = 0;
        loop {
            let key = self.read_key();
            if key.enter {
                return pos;
            }
            if key.backspace {
                if pos > 0 {
                    pos -= 1;
                }
                continue;
            }
            if let Some(c) = key.ascii {
                if pos < buf.len() - 1 {
                    buf[pos] = c;
                    pos += 1;
                }
            }
        }
    }
}
