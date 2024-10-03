ORG 0x7C00                                ; Boot sector origin
BITS 16

CODE_SEG equ gdt_code - gdt_start
DATA_SEG equ gdt_data - gdt_start

_start:
    jmp short start
    nop

times 33 db 0                             ; Fill the rest of the sector with 0 for boot signature

start:
    jmp 0:next                            ; Far jump to the next instruction in 16-bit mode

next:
    cli                                   ; Clear interrupts
    xor AX, AX
    mov DS, AX
    mov ES, AX
    mov SS, AX
    mov SP, 0x7C00                        ; Set the stack pointer
    sti                                   ; Enable interrupts

.load_protected_mode:
    cli                                   ; Clear interrupts
    lgdt [gdt_descriptor]                 ; Load the GDT
    mov EAX, CR0
    or EAX, 0x1                           ; Set the first bit of CR0 to enable protected mode
    mov CR0, EAX
    jmp CODE_SEG:load32                   ; Far jump to switch to protected mode


gdt_start:

gdt_null:
    dd 0x0                                ; First 32 bits of base address
    dd 0x0                                ; First 32 bits of segment limit

gdt_code:                                 ; Code segment for 64-bit (0x10 offset)
    dw 0xFFFF                             ; Segment limit (low 16 bits)
    dw 0x0                                ; Base address (low 16 bits)
    db 0x0                                ; Base address (next 8 bits)
    db 0x9A                               ; Access byte (executable, readable, accessed)
    db 10101111b                          ; Granularity, 64-bit, and size flags
    db 0x0                                ; Base address (last 8 bits)

gdt_data:                                 ; Data segment (0x18 offset)
    dw 0xFFFF                             ; Segment limit
    dw 0x0                                ; Base address
    db 0x0                                ; Base address
    db 0x92                               ; Access byte (readable/writable, accessed)
    db 11001111b                          ; Granularity and size flags
    db 0x0                                ; Base address

gdt_end:

gdt_descriptor:
    dw gdt_end - gdt_start - 1            ; Size of GDT
    dd gdt_start                          ; Base address of GDT

[BITS 32]
load32:
    mov EAX, cr4
    or EAX, 0x20                          ; Enable PAE (Physical Address Extension)
    mov cr4, EAX

    mov ECX, 0xC0000080                   ; Load MSR for long mode
    rdmsr
    or EAX, 0x100                         ; Set the long mode enable (LME) bit
    wrmsr

    mov EAX, cr0
    or EAX, 0x80000001                    ; Enable paging and protected mode
    mov cr0, EAX

    jmp CODE_SEG:load64                   ; Far jump to switch to long mode (64-bit)


[BITS 64]
load64:
    mov RAX, 1                            ; Simple 64-bit mode test
    mov RCX, 100                          ; Example value
    mov RDI, 0x0100000                    ; Destination address in memory
    call ata_lba_read                     ; Load kernel via ATA LBA read
    jmp 0x100000                          ; Jump to kernel entry point (64-bit)



ata_lba_read:
    mov RBX, RAX
    shr RAX, 24
    or RAX, 0xE0
    mov DX, 0x1F6
    out DX, al

    mov EAX, ECX
    mov DX, 0x1F2
    out DX, AL

    mov EAX, EBX
    mov DX, 0x1F3
    out DX, AL

    mov DX, 0x1F4
    mov EAX, EBX
    shr EAX, 8
    out DX, AL

    mov DX, 0x1F5
    mov EAX, EBX
    shr EAX, 16
    out DX, AL

    mov DX, 0x1F7
    mov AL, 0x20
    out DX, AL

.next_sector:
    push RCX

.try_again:
    mov DX, 0x1F7
    in AL, DX
    test AL, 8
    jz .try_again

    mov RCX, 256
    mov DX, 0x1F0
    rep insw
    pop RCX
    loop .next_sector
    ret


times 510 - ($ - $$) db 0                 ; Pad with zeros to make up 512 bytes
dw 0xAA55                                 ; Boot signature (0xAA55)
