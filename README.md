# OmnOS

OmnOS is an adaptive operating system that scales down to constrained hardware
and up to high-core-count systems from one kernel architecture.
It boots small by default, then enables only the capabilities the machine can actually use.

OmnOS does not have minimum requirements. It has adaptive modes.

| RAM | Mode | Scheduler | Services |
|---|---|---|---|
| 16-512 MB | Nano | Cooperative | Minimal |
| 512 MB - 4 GB | Lite | Round-robin | Essential |
| 4-32 GB | Desktop | Priority | Full |
| 32 GB+ / many-core | Scale | Multi-queue | Server |

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

## Current features (v0.0.2)

- **Multiboot-compliant boot** with 32→64-bit transition (page tables, PAE, long mode)
- **VGA text mode** driver (80x25, fmt::Write)
- **Serial port** output (COM1, 115200 8n1) for QEMU debugging
- **Multiboot info parser** — memory map, bootloader name, command line
- **CPUID detection** — vendor, family/model/stepping, core count, SSE/AVX/VMX flags
- **Adaptive profile** — Nano/Lite/Desktop/Scale selected at boot from RAM + cores
- **PS/2 keyboard** driver (scancode set 1, shift-aware)
- **Interactive terminal** with commands: `help`, `sysinfo`, `mem`, `cpu`, `profile`, `clear`, `reboot`
- **Dual output** — all command output goes to VGA and serial simultaneously

## Design

- **Single-layer kernel** written in Rust (no_std, nightly, no external crates)
- **Adaptive execution profile** chosen at boot based on CPUID + RAM
- **Multiboot** BIOS boot; UEFI planned
- **WASM runtime** for userspace applications (future)
- **Sandboxed drivers** in userspace (future)

## Implementation notes

- Boot code in `boot/boot.s` (GAS AT&T syntax, assembled via llvm-mc)
- Build script (`build.rs`) compiles boot assembly and produces `libboot.a`
- Custom linker script (`linker.ld`) at 1MB base address
- `ld.lld` links boot + Rust staticlib into final ELF
- `llvm-objcopy --strip-debug` removes debug sections for the bootable binary

## Roadmap

1. ~~Multiboot info parser~~ (v0.0.2)
2. ~~Serial logger~~ (v0.0.2)
3. ~~CPUID + adaptive profile~~ (v0.0.2)
4. ~~PS/2 keyboard + terminal~~ (v0.0.2)
5. GDT / IDT / exception handling
6. Physical memory manager (bitmap)
7. Heap allocator (alloc, Vec, String)
8. Timer IRQ + cooperative scheduler (Nano mode)
9. PCI enumeration
10. Framebuffer graphics mode
