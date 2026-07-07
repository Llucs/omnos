.code32
.section .multiboot, "a"
.align 4
.long 0x1BADB002
.long 0
.long -(0x1BADB002)

.section .text
.global start

start:
    movl $stack_top, %esp

    leal page_table_l3, %eax
    orl $3, %eax
    movl %eax, page_table_l4

    leal page_table_l2, %eax
    orl $3, %eax
    movl %eax, page_table_l3

    xorl %ecx, %ecx
    movl $0x83, %eax
1:
    movl %eax, page_table_l2(,%ecx,8)
    addl $0x200000, %eax
    incl %ecx
    cmpl $2, %ecx
    jne 1b

    movl %cr4, %eax
    orl $(1 << 5), %eax
    movl %eax, %cr4

    movl $0xC0000080, %ecx
    rdmsr
    orl $(1 << 8), %eax
    wrmsr

    leal page_table_l4, %eax
    movl %eax, %cr3

    movl %cr0, %eax
    orl $(1 << 31), %eax
    movl %eax, %cr0

    lgdt gdt64_pointer

    ljmp $8, $long_mode_entry

.code64
long_mode_entry:
    movw $16, %ax
    movw %ax, %ds
    movw %ax, %es
    movw %ax, %fs
    movw %ax, %gs
    movw %ax, %ss

    movl %eax, %edi
    movl %ebx, %esi
    call _start

.hang:
    hlt
    jmp .hang

.section .bss
.align 4096
page_table_l4:
    .space 4096
page_table_l3:
    .space 4096
page_table_l2:
    .space 4096
stack_bottom:
    .space 32768
stack_top:

.section .rodata
.align 8
gdt64:
    .quad 0
    .quad (1 << 43) | (1 << 44) | (1 << 47) | (1 << 53)
    .quad (1 << 44) | (1 << 47) | (1 << 41)
gdt64_pointer:
    .word gdt64_pointer - gdt64 - 1
    .long gdt64
