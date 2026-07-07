use crate::cpuid;
use core::fmt;

pub enum Profile {
    Nano,
    Lite,
    Desktop,
    Scale,
}

impl Profile {
    pub fn detect(memory_mib: u32, cpu_info: &cpuid::CpuInfo) -> Self {
        if memory_mib < 512 {
            Profile::Nano
        } else if memory_mib < 4096 {
            Profile::Lite
        } else if memory_mib < 32768 || cpu_info.cores < 32 {
            Profile::Desktop
        } else {
            Profile::Scale
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Profile::Nano => "nano",
            Profile::Lite => "lite",
            Profile::Desktop => "desktop",
            Profile::Scale => "scale",
        }
    }

    pub fn scheduler(&self) -> &'static str {
        match self {
            Profile::Nano => "coop",
            Profile::Lite => "round-robin",
            Profile::Desktop => "priority",
            Profile::Scale => "multi-queue",
        }
    }

    pub fn services(&self) -> &'static str {
        match self {
            Profile::Nano => "minimal",
            Profile::Lite => "basic",
            Profile::Desktop => "full",
            Profile::Scale => "distributed",
        }
    }
}

pub struct SystemInfo {
    pub cpu: cpuid::CpuInfo,
    pub memory_mib: u32,
    pub profile: Profile,
}

impl SystemInfo {
    pub fn detect(memory_mib: u32) -> Self {
        let cpu = cpuid::detect();
        let profile = Profile::detect(memory_mib, &cpu);
        SystemInfo {
            cpu,
            memory_mib,
            profile,
        }
    }

    pub fn display(&self, w: &mut impl fmt::Write) {
        let vendor_str = core::str::from_utf8(&self.cpu.vendor).unwrap_or("unknown");
        let _ = writeln!(w, "OmnOS v0.0.1 [profile: {}]", self.profile.name());
        let _ = writeln!(
            w,
            "  CPU: {} family={} model={} stepping={} cores={}",
            vendor_str,
            self.cpu.family,
            self.cpu.model,
            self.cpu.stepping,
            self.cpu.cores,
        );
        let _ = writeln!(w, "  Memory: {} MiB", self.memory_mib);
        let _ = writeln!(
            w,
            "  Scheduler: {} | Services: {}",
            self.profile.scheduler(),
            self.profile.services(),
        );
    }
}
