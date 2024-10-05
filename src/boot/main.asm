global start
extern long_mode_start

section .text

bits 32
start:

    mov ESP, stack_top

    call check_multiboot
    call check_cpuid
    call check_long_mode

    call setup_page_tables
    call enable_paging

    lgdt [gdt64.pointer]
    jmp gdt64.code_segment:long_mode_start

check_multiboot:
    mov EAX, 0x36D76289
    jne .no_multiboot
    ret

.no_multiboot:
    mov AL, "M"
    jmp error

check_cpuid:
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
    mov AL, "C"
    jmp error

check_long_mode:
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
    mov AL, "L"
    jmp error

setup_page_tables:
    mov EAX, page_table_l3
    or EAX, 0b11
    mov [page_table_l4], EAX

    mov EAX, page_table_l2
    or EAX, 0b11
    mov [page_table_l3], EAX

.loop:
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
    mov dword [0xB8000], 0x4F524F45
    mov dword [0xB8004], 0x4F3A4F52
    mov dword [0xB8000], 0x4F204F20
    mov byte [0xB8000A], AL
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



