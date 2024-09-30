ORG 0x7C00                                ; boot sector origin
BITS 16

CODE_SEG equ gdt_code - gdt_start
DATA_SEG equ gdt_data - gdt_start

_start:
    jmp short start
    nop

times 33 db 0                             ; fill the rest of the sector with 0 for boot signature

start:
    jmp 0:next

next:
    cli                                   
    mov AX, 0x00
    mov DS, AX
    mov ES, AX
    mov SS, AX
    mov SP, 0x7C00
    sti

.load_protected_mode:
    cli
    lgdt [gdt_descriptor]                 ; load the gdt
    mov EAX, CR0
    or EAX, 0x1                           ; set the first bit of CR0 to 1
    mov CR0, EAX
    jmp CODE_SEG:load32                ; jump to the next instruction in 32-bit mode


gdt_start:

gdt_null:
    dd 0x0                                ; first 32 bits of base address
    dd 0x0                                ; first 32 bits of segment limit

gdt_code:                                 ; should point to CS, 0x10 offset
    dw 0xFFFF                             ; first 16 bits of segment limit
    dw 0x0                                ; first 16 bits of base address
    db 0x0                                ; next 8 bits of base address
    db 0x9A                               ; access byte
    db 11001111b                          ; high-low 4-bit flags
    db 0x0                                ; last 8 bits of base address

gdt_data:                                 ; should point to DS, SS, ES, GS, FS
    dw 0xFFFF
    dw 0x0
    db 0x0
    db 0x92
    db 11001111b
    db 0x0

gdt_end:

gdt_descriptor:
    dw gdt_end - gdt_start - 1            ; size of gdt
    dd gdt_start                          ; base address of gdt

[BITS 32]
load32:
    mov EAX, 1
    mov ECX, 100
    mov EDI, 0x0100000
    call ata_lba_read
    jmp CODE_SEG:0x100000

ata_lba_read:
    mov EBX, EAX
    shr EAX, 24
    or EAX, 0xE0
    mov DX, 0x1F6
    out DX, AL

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
    push ECX

.try_again:
    mov DX, 0x1F7
    in AL, DX
    test AL, 8
    jz .try_again

    mov ECX, 256
    mov DX, 0x1F0
    rep insw
    pop ECX
    loop .next_sector
    ret


times 510 - ($ - $$) db 0                 ; fill the rest of the sector with 0 for boot signature
dw 0xAA55                                 ; boot signature (little endian byte order for x86)
 
;Boot sector that loads the GDT and switches to protected mode.