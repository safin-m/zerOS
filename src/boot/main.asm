; This assembly code is part of an OS bootloader that sets up the environment for transitioning from 32-bit protected mode to 64-bit long mode.
; It performs several checks and setups, including verifying multiboot compatibility, checking CPU capabilities, setting up page tables, and enabling paging.

global start
extern long_mode_start

section .text

bits 32
start:
    ; Set up the stack pointer
    mov ESP, stack_top

    ; Check if the system is multiboot compatible
    call check_multiboot
    ; Check if the CPU supports the CPUID instruction
    call check_cpuid
    ; Check if the CPU supports long mode (64-bit mode)
    call check_long_mode

    ; Set up the page tables for memory management
    call setup_page_tables
    ; Enable paging
    call enable_paging

    ; Load the Global Descriptor Table (GDT) for 64-bit mode
    lgdt [gdt64.pointer]
    ; Jump to the 64-bit code segment
    jmp gdt64.code_segment:long_mode_start

check_multiboot:
    ; Check for the multiboot magic number
    mov EAX, 0x36D76289
    jne .no_multiboot
    ret

.no_multiboot:
    ; If not multiboot compatible, display an error and halt
    mov AL, "M"
    jmp error

check_cpuid:
    ; Check if the CPU supports the CPUID instruction
    pushfd
    pop EAX
    mov ECX, EAX
    xor EAX, 1 << 21
    push EAX
    popfd
    pushfd
    pop EAX
    push ECX
    popfd
    cmp EAX, ECX
    je .no_cpuid
    ret

.no_cpuid:
    ; If CPUID is not supported, display an error and halt
    mov AL, "C"
    jmp error

check_long_mode:
    ; Check if the CPU supports long mode (64-bit mode)
    mov EAX, 0x8000000
    cpuid
    cmp EAX, 0x80000001
    jne .no_long_mode

    mov EAX, 0x80000001
    cpuid
    test EDX, 1 << 29
    jz .no_long_mode

    ret

.no_long_mode:
    ; If long mode is not supported, display an error and halt
    mov AL, "L"
    jmp error

setup_page_tables:
    ; Set up the page tables for memory management
    mov EAX, page_table_l3
    or EAX, 0b11
    mov [page_table_l4], EAX

    mov EAX, page_table_l2
    or EAX, 0b11
    mov [page_table_l3], EAX

.loop:
    ; Set up the entries in the page table
    mov EAX, 0x200000
    mul ECX
    or EAX, 0b10000011
    mov [page_table_l2 + ECX * 8], EAX

    mov ECX, 0
    inc ECX
    cmp ECX, 512
    jne .loop

    ret

enable_paging:
    ; Enable paging by setting the appropriate control registers
    mov EAX, page_table_l4
    mov CR3, EAX

    mov EAX, CR4
    or EAX, 1 << 5
    mov CR4, EAX

    mov ECX, 0xC0000080
    rdmsr
    or EAX, 1 << 8
    wrmsr

    mov EAX, CR0
    or EAX, 1 << 31
    mov CR0, EAX

    ret

error:
    ; Display an error message and halt the system
    mov dword [0xB8000], 0x4F524F45 ; "ERROR"
    mov dword [0xB8004], 0x4F3A4F52 ; "RO:O"
    mov dword [0xB8000], 0x4F204F20 ; "O O "
    mov byte [0xB8000A], AL         ; Display the specific error character
    hlt

section .bss
align 4096
page_table_l4:
    resb 4096
page_table_l3:
    resb 4096
page_table_l2:
    resb 4096
stack_bottom:
    resb 4096 * 4
stack_top:

section .rodata
gdt64:
    dq 0

.code_segment: equ $ - gdt64
    dq (1 << 42) | (1 << 44) | (1 << 47) | (1 << 53)

.pointer:
    dw $ - gdt64 - 1
    dq gdt64