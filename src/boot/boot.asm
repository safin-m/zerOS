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
    jmp CODE_SEG:load32                   ; jump to the next instruction in 32-bit mode


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
    mov AX, DATA_SEG
    mov DS, AX
    mov ES, AX
    mov FS, AX
    mov GS, AX
    mov SS, AX
    mov ebp, 0x00200000                  ; set the base pointer to 2MB
    mov esp, ebp                         ; set the stack pointer to 2MB

    in AL, 0x92                          ; read the value of the keyboard controller
    or AL, 2
    out 0x92, AL                         ; write the value back to the keyboard controller
    
    jmp $

times 510 - ($ - $$) db 0                 ; fill the rest of the sector with 0 for boot signature
dw 0xAA55                                 ; boot signature (little endian byte order for x86)
 
 ;Boot sector that loads the GDT and switches to protected mode. 
 ;To compile the code, we need to use the nasm assembler. 
 ;nasm -f bin ./boot.asm -o ./boot.bin
 
 ;The command above will generate a binary file called boot.bin. 
 ;Now we need to create a disk image and copy the boot.bin file to it. 
 ;dd if=/dev/zero of=disk.img bs=512 count=2880