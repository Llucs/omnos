use core::arch::asm;

pub struct CpuInfo {
    pub vendor: [u8; 12],
    pub family: u8,
    pub model: u8,
    pub stepping: u8,
    pub cores: u8,
    pub has_sse: bool,
    pub has_sse2: bool,
    pub has_sse3: bool,
    pub has_ssse3: bool,
    pub has_sse4_1: bool,
    pub has_sse4_2: bool,
    pub has_avx: bool,
    pub has_avx2: bool,
    pub has_fpu: bool,
    pub has_msr: bool,
    pub has_apic: bool,
    pub has_x2apic: bool,
    pub has_vmx: bool,
}

fn cpuid(leaf: u32, subleaf: u32) -> (u32, u32, u32, u32) {
    let mut eax = leaf;
    let mut ecx = subleaf;
    let ebx;
    let edx;
    unsafe {
        asm!(
            "push rbx",
            "cpuid",
            "mov {0:e}, ebx",
            "pop rbx",
            out(reg) ebx,
            inout("eax") eax,
            inout("ecx") ecx,
            out("edx") edx,
        );
    }
    (eax, ebx, ecx, edx)
}

pub fn detect() -> CpuInfo {
    let (max_leaf, b, c, d) = cpuid(0, 0);

    let vendor = [
        (b >> 0) as u8, (b >> 8) as u8, (b >> 16) as u8, (b >> 24) as u8,
        (d >> 0) as u8, (d >> 8) as u8, (d >> 16) as u8, (d >> 24) as u8,
        (c >> 0) as u8, (c >> 8) as u8, (c >> 16) as u8, (c >> 24) as u8,
    ];

    let mut family = 0u8;
    let mut model = 0u8;
    let mut stepping = 0u8;
    let mut cores = 1u8;
    let mut has_sse = false;
    let mut has_sse2 = false;
    let mut has_sse3 = false;
    let mut has_ssse3 = false;
    let mut has_sse4_1 = false;
    let mut has_sse4_2 = false;
    let mut has_avx = false;
    let mut has_avx2 = false;
    let mut has_fpu = false;
    let mut has_msr = false;
    let mut has_apic = false;
    let mut has_x2apic = false;
    let mut has_vmx = false;

    if max_leaf >= 1 {
        let (a, b1, c1, d1) = cpuid(1, 0);

        stepping = (a & 0xF) as u8;
        model = ((a >> 4) & 0xF) as u8;
        family = ((a >> 8) & 0xF) as u8;

        if family == 0xF {
            family += ((a >> 20) & 0xFF) as u8;
        }
        if family == 0x6 || family == 0xF {
            model |= ((a >> 12) & 0xF0) as u8;
        }

        has_fpu = (d1 & (1 << 0)) != 0;
        has_msr = (d1 & (1 << 5)) != 0;
        has_apic = (d1 & (1 << 9)) != 0;
        has_sse = (d1 & (1 << 25)) != 0;
        has_sse2 = (d1 & (1 << 26)) != 0;

        has_vmx = (c1 & (1 << 5)) != 0;
        has_sse3 = (c1 & (1 << 0)) != 0;
        has_ssse3 = (c1 & (1 << 9)) != 0;
        has_sse4_1 = (c1 & (1 << 19)) != 0;
        has_sse4_2 = (c1 & (1 << 20)) != 0;
        has_x2apic = (c1 & (1 << 21)) != 0;
        has_avx = (c1 & (1 << 28)) != 0;

        let logical = ((b1 >> 16) & 0xFF) as u8;
        if logical > cores {
            cores = logical;
        }
    }

    if max_leaf >= 7 {
        let (_, b7, _, _) = cpuid(7, 0);
        has_avx2 = (b7 & (1 << 5)) != 0;
    }

    if max_leaf >= 0xB {
        let (_, b_top, c_top, _) = cpuid(0xB, 0);
        if c_top != 0 {
            let (_, b_core, _, _) = cpuid(0xB, 1);
            if b_core & 0xFFFF != 0 {
                cores = (b_core & 0xFFFF) as u8;
            } else {
                cores = (b_top & 0xFFFF) as u8;
            }
        }
    }

    if cores == 1 && max_leaf >= 4 {
        let (a4, _, _, _) = cpuid(4, 0);
        let pkg_cores = ((a4 >> 26) & 0x3F) + 1;
        if pkg_cores > 1 {
            cores = pkg_cores as u8;
        }
    }

    if cores > 128 {
        cores = 128;
    }

    CpuInfo {
        vendor,
        family,
        model,
        stepping,
        cores,
        has_sse,
        has_sse2,
        has_sse3,
        has_ssse3,
        has_sse4_1,
        has_sse4_2,
        has_avx,
        has_avx2,
        has_fpu,
        has_msr,
        has_apic,
        has_x2apic,
        has_vmx,
    }
}
