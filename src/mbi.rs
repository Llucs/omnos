use core::ptr;

#[repr(C)]
struct Raw {
    flags: u32,
    mem_lower: u32,
    mem_upper: u32,
    boot_device: u32,
    cmdline: u32,
    mods_count: u32,
    mods_addr: u32,
    syms: [u32; 4],
    mmap_length: u32,
    mmap_addr: u32,
    drives_length: u32,
    drives_addr: u32,
    config_table: u32,
    bootloader_name: u32,
}

pub struct MultibootInfo {
    inner: &'static Raw,
}

impl MultibootInfo {
    pub fn flags(&self) -> u32 {
        self.inner.flags
    }

    pub fn mem_upper(&self) -> u32 {
        self.inner.mem_upper
    }

    pub fn mem_upper_mib(&self) -> u32 {
        self.inner.mem_upper / 1024
    }

    pub fn bootloader_name(&self) -> Option<&'static str> {
        if self.inner.flags & (1 << 9) == 0 {
            return None;
        }
        unsafe { cstr_at(self.inner.bootloader_name as *const u8) }
    }

    pub fn cmdline(&self) -> Option<&'static str> {
        if self.inner.flags & (1 << 0) == 0 {
            return None;
        }
        unsafe { cstr_at(self.inner.cmdline as *const u8) }
    }

    pub fn memory_map_entries(&self) -> MemoryMapIter {
        if self.inner.flags & (1 << 6) == 0 {
            return MemoryMapIter { current: core::ptr::null(), end: core::ptr::null() };
        }
        let start = self.inner.mmap_addr as *const u8;
        let len = self.inner.mmap_length as usize;
        let end = unsafe { start.add(len) };
        MemoryMapIter { current: start, end }
    }
}

unsafe fn cstr_at<'a>(ptr: *const u8) -> Option<&'a str> {
    if ptr.is_null() {
        return None;
    }
    let mut len = 0;
    while unsafe { ptr::read(ptr.add(len)) } != 0 {
        len += 1;
    }
    let slice = unsafe { core::slice::from_raw_parts(ptr, len) };
    core::str::from_utf8(slice).ok()
}

pub fn parse(ptr: *const u8) -> Option<MultibootInfo> {
    if ptr.is_null() {
        return None;
    }
    unsafe {
        Some(MultibootInfo {
            inner: &*(ptr as *const Raw),
        })
    }
}

pub struct MemoryMapEntry {
    base_addr: u64,
    length: u64,
    ty: u32,
}

impl MemoryMapEntry {
    pub fn base_addr(&self) -> u64 {
        self.base_addr
    }

    pub fn length(&self) -> u64 {
        self.length
    }

    pub fn is_usable(&self) -> bool {
        self.ty == 1
    }
}

pub struct MemoryMapIter {
    current: *const u8,
    end: *const u8,
}

impl Iterator for MemoryMapIter {
    type Item = MemoryMapEntry;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.is_null() || self.current >= self.end {
            return None;
        }
        let sz = unsafe { ptr::read_unaligned::<u32>(self.current as *const u32) };
        let base_lo = unsafe { ptr::read_unaligned::<u32>(self.current.add(4) as *const u32) };
        let base_hi = unsafe { ptr::read_unaligned::<u32>(self.current.add(8) as *const u32) };
        let len_lo = unsafe { ptr::read_unaligned::<u32>(self.current.add(12) as *const u32) };
        let len_hi = unsafe { ptr::read_unaligned::<u32>(self.current.add(16) as *const u32) };
        let ty = unsafe { ptr::read_unaligned::<u32>(self.current.add(20) as *const u32) };
        let base_addr = (base_hi as u64) << 32 | base_lo as u64;
        let length = (len_hi as u64) << 32 | len_lo as u64;
        let entry_size = if sz > 0 { 4 + sz as usize } else { 24 };
        self.current = unsafe { self.current.add(entry_size) };
        Some(MemoryMapEntry { base_addr, length, ty })
    }
}
