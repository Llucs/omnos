VERSION := 0.0.1
CARGO := cargo +nightly
TARGET := x86_64-unknown-none

BUILD_DIR := target/$(TARGET)/release
KERNEL_A := $(BUILD_DIR)/libomnos.a
ELF := $(BUILD_DIR)/omnos.elf
ELF_STRIPPED := $(BUILD_DIR)/omnos.bin
ISO := $(BUILD_DIR)/omnos.iso

.PHONY: all kernel elf iso clean

all: elf

kernel:
	$(CARGO) build --release

elf: kernel
	BOOT_A=$$(find $(BUILD_DIR) -name "libboot.a" | head -1); \
	ld.lld -m elf_x86_64 -T linker.ld -e start -o $(ELF) "$$BOOT_A" $(KERNEL_A); \
	llvm-objcopy --strip-debug $(ELF) $(ELF_STRIPPED)
	@echo "OmnOS $(VERSION) :: $(ELF_STRIPPED)"

iso: elf
	mkdir -p $(BUILD_DIR)/iso/boot/grub
	cp $(ELF_STRIPPED) $(BUILD_DIR)/iso/boot/omnos.bin
	printf "set timeout=0\nmenuentry \"OmnOS $(VERSION)\" {\n  multiboot /boot/omnos.bin\n}\n" \
	    > $(BUILD_DIR)/iso/boot/grub/grub.cfg
	grub-mkrescue -o $(ISO) $(BUILD_DIR)/iso 2>/dev/null || \
	    xorriso -as mkisofs -b boot/grub/eltorito.img \
	        -no-emul-boot -boot-load-size 4 -boot-info-table \
	        -o $(ISO) $(BUILD_DIR)/iso 2>/dev/null || \
	    echo "ISO tools not available. ELF at $(ELF_STRIPPED)"
	rm -rf $(BUILD_DIR)/iso

run: elf
	qemu-system-x86_64 -kernel $(ELF_STRIPPED)

clean:
	$(CARGO) clean
	rm -f $(ELF) $(ELF_STRIPPED) $(ISO)
