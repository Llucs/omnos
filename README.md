# OmnOS

OmnOS is an adaptive operating system that scales down to constrained hardware
and up to high-core-count systems from one kernel architecture.
It boots small by default, then enables only the capabilities the machine can actually use.

OmnOS does not have minimum requirements. It has adaptive modes.

| RAM | Mode | Scheduler | Services |
|---|---|---|---|
| 16-512 MB | Nano | Cooperative | Minimal |
| 512 MB - 4 GB | Lite | Hybrid | Essential |
| 4-32 GB | Desktop | Preemptive | Full |
| 32 GB+ / many-core | Scale | SMP / NUMA | Server |

## Build

```
rustup target add x86_64-unknown-none
make elf
```

## Run (QEMU)

```
make run
```

## Boot on hardware

```
make iso
```

Boot the ISO via BIOS on real hardware. Or:

```
qemu-system-x86_64 -kernel target/x86_64-unknown-none/release/omnos.bin
```

## Design

- **Single-layer kernel** written in Rust (no_std, nightly)
- **Adaptive execution profile** chosen at boot based on CPUID + RAM + capabilities
- **Multiboot-compliant** BIOS boot; UEFI planned
- **WASM runtime** for userspace applications (future)
- **Sandboxed drivers** in userspace (future)

## Roadmap

1. Multiboot info parser (memory map, framebuffer, bootloader)
2. Serial logger
3. CPUID + adaptive profile selection
4. GDT / IDT / exception handling
5. Physical memory manager
6. Heap allocator (alloc, Vec, String)
7. Timer IRQ + cooperative scheduler (Nano mode)
8. Minimal terminal (sysinfo, mem, clear, help)
9. PCI enumeration
10. Framebuffer graphics mode
