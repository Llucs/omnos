# OmnOS

Adaptive kernel that molds to the hardware instead of demanding hardware mold to it.

OmnOS does not have minimum requirements. It has adaptive modes.

| RAM | Mode |
|---|---|
| 16-512 MB | Nano |
| 512 MB - 4 GB | Lite |
| 4-32 GB | Desktop |
| 32 GB+ / many-core | Scale |

## Build

rustup target add x86_64-unknown-none
make elf

## Run (QEMU)

make run

## Boot on hardware

make iso

Boot the ISO via BIOS (UEFI not yet supported). Or:

qemu-system-x86_64 -kernel target/x86_64-unknown-none/release/omnos.bin
