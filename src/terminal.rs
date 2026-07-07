use core::fmt;
use core::fmt::Write;
use crate::vga;
use crate::serial;
use crate::keyboard;
use crate::mbi;
use crate::cpuid;
use crate::profile;

struct LineReader<'a> {
    kbd: keyboard::Keyboard,
    buf: &'a mut [u8],
    pos: usize,
}

impl<'a> LineReader<'a> {
    fn new(buf: &'a mut [u8]) -> Self {
        LineReader { kbd: keyboard::Keyboard::new(), buf, pos: 0 }
    }

    fn read(&mut self, echo: &mut impl fmt::Write) -> &str {
        self.pos = 0;
        loop {
            let key = self.kbd.read_key();
            if key.enter {
                let _ = writeln!(echo);
                break;
            }
            if key.backspace {
                if self.pos > 0 {
                    self.pos -= 1;
                    let _ = write!(echo, "\x08 \x08");
                }
                continue;
            }
            if let Some(c) = key.ascii {
                if self.pos < self.buf.len().saturating_sub(1) {
                    self.buf[self.pos] = c;
                    self.pos += 1;
                    let _ = write!(echo, "{}", c as char);
                }
            }
        }
        core::str::from_utf8(&self.buf[..self.pos]).unwrap_or("")
    }
}

fn cmd_sysinfo(vga: &mut vga::Writer, serial: &mut serial::Writer, mb_info: Option<&mbi::MultibootInfo>) {
    let mem = mb_info.map(|m| m.mem_upper_mib()).unwrap_or(0);
    let sys = profile::SystemInfo::detect(mem);
    let _ = sys.display(vga);
    let _ = sys.display(serial);
    if let Some(mbi) = mb_info {
        if let Some(name) = mbi.bootloader_name() {
            let _ = writeln!(vga, "  Bootloader: {}", name);
        }
    }
}

fn cmd_mem(vga: &mut vga::Writer, serial: &mut serial::Writer, mb_info: Option<&mbi::MultibootInfo>) {
    if let Some(mbi) = mb_info {
        let mem = mbi.mem_upper_mib();
        let _ = writeln!(vga, "  Memory: {} MiB total", mem);
        let _ = writeln!(serial, "  Memory: {} MiB total", mem);
        for entry in mbi.memory_map_entries() {
            let ty = if entry.is_usable() { "usable" } else { "reserved" };
            let _ = writeln!(vga, "  {:#010x} - {:#010x} ({})",
                entry.base_addr() as usize,
                (entry.base_addr() + entry.length()) as usize,
                ty);
        }
    } else {
        let _ = writeln!(vga, "  No MBI available");
    }
}

fn cmd_profile(vga: &mut vga::Writer, serial: &mut serial::Writer, mb_info: Option<&mbi::MultibootInfo>) {
    let mem = mb_info.map(|m| m.mem_upper_mib()).unwrap_or(0);
    let cpu = cpuid::detect();
    let p = profile::Profile::detect(mem, &cpu);
    let _ = writeln!(vga, "  Mode:       {}", p.name());
    let _ = writeln!(vga, "  Scheduler:  {}", p.scheduler());
    let _ = writeln!(vga, "  Services:   {}", p.services());
    let _ = writeln!(serial, "  Mode:       {}", p.name());
}

fn cmd_cpu(vga: &mut vga::Writer, serial: &mut serial::Writer) {
    let cpu = cpuid::detect();
    let vendor = core::str::from_utf8(&cpu.vendor).unwrap_or("?");
    let _ = writeln!(vga, "  Vendor:     {}", vendor);
    let _ = writeln!(vga, "  Family:     {}", cpu.family);
    let _ = writeln!(vga, "  Model:      {}", cpu.model);
    let _ = writeln!(vga, "  Stepping:   {}", cpu.stepping);
    let _ = writeln!(vga, "  Cores:      {}", cpu.cores);
    let _ = writeln!(vga, "  SSE4.1:     {}", if cpu.has_sse4_1 { "yes" } else { "no" });
    let _ = writeln!(vga, "  AVX:        {}", if cpu.has_avx { "yes" } else { "no" });
    let _ = writeln!(serial, "  CPU: {} f={} m={} s={} cores={}", vendor, cpu.family, cpu.model, cpu.stepping, cpu.cores);
}

fn cmd_help(vga: &mut vga::Writer, serial: &mut serial::Writer) {
    let cmds = [
        "help      Show this message",
        "sysinfo   Display system information",
        "mem       Show memory map",
        "cpu       Show CPU details",
        "profile   Show adaptive mode",
        "clear     Clear screen",
        "reboot    Reboot the system",
    ];
    for c in cmds {
        let _ = writeln!(vga, "  {}", c);
        let _ = writeln!(serial, "  {}", c);
    }
}

pub fn shell(vga: &mut vga::Writer, mb_info_ptr: *const u8) -> ! {
    let mut serial_w = serial::writer();
    let mut line_buf = [0u8; 128];

    let _ = writeln!(vga, "OmnOS Terminal :: type 'help' for commands");
    let _ = writeln!(vga);

    loop {
        let _ = write!(vga, "omnos> ");
        let _ = write!(serial_w, "omnos> ");

        let mut reader = LineReader::new(&mut line_buf);
        let input = reader.read(vga);

        let mb_info = mbi::parse(mb_info_ptr);

        match input {
            "help" => cmd_help(vga, &mut serial_w),
            "sysinfo" => cmd_sysinfo(vga, &mut serial_w, mb_info.as_ref()),
            "mem" => cmd_mem(vga, &mut serial_w, mb_info.as_ref()),
            "cpu" => cmd_cpu(vga, &mut serial_w),
            "profile" => cmd_profile(vga, &mut serial_w, mb_info.as_ref()),
            "clear" => {
                vga.clear();
                let _ = writeln!(serial_w, "[screen cleared]");
            }
            "reboot" => {
                let _ = writeln!(vga, "Rebooting...");
                let _ = writeln!(serial_w, "Rebooting...");
                unsafe {
                    core::ptr::write_volatile(0x64 as *mut u8, 0xFEu8);
                }
                loop {}
            }
            "" => {}
            _ => {
                let _ = writeln!(vga, "  Unknown command: {}", input);
                let _ = writeln!(serial_w, "  Unknown command: {}", input);
            }
        }
    }
}
