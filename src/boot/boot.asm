ORG 0                     ; boot sector origin
BITS 16                   ; 16 bit code

_start:
    jmp short start       ; jump to start label
    nop

times 33 db 0             ; fill the rest of the sector with 0 for boot signature

start:                    ; bios routine
    jmp 0x7C0:next        ; jump to start label

next:
    cli                   ; clear interrupts
    mov AX, 0x07C0        ; set data segment to 0x07C0
    mov DS, AX            ; set data segment to 0x07C0
    mov ES, AX            ; set extra segment to 0x07C0
    mov AX, 0x00          ; set stack segment to 0x0000
    mov SS, AX            ; set stack segment to 0x0000
    mov SP, 0x7C00        ; set stack pointer to 0x7C00
    sti                   ; set interrupts
    
    mov SI, msg           ; set SI to point to the message
    call print            ; call the print function
    jmp $                 ; infinite loop to stop the cpu from executing random code after the boot sector

print:
    mov BX, 0             ; set video memory address to 0
.loop:
    lodsb                 ; load byte from SI to AL
    cmp AL, 0             ; check if AL is 0 (end of string)
    je .done              ; if AL is 0, jump to done
    call print_msg        ; call print_msg function
    jmp .loop             ; loop back to load next character
.done:
    ret                   ; return from the function

print_msg:
    mov AH, 0EH           ; set teletype function
    int 0x10              ; call bios interrupt to print character
    ret                   ; return from the function

msg: db 'Bootloader', 0   ; message to print


times 510 - ($ - $$) db 0   ; fill the rest of the sector with 0 for boot signature
dw 0xAA55                 ; boot signature (little endian byte order for x86)
 
 ;The code is simple, it prints the message on the screen with a background color of 0. 
 ;To compile the code, we need to use the nasm assembler. 
 ;nasm -f bin ./boot.asm -o ./boot.bin
 
 ;The command above will generate a binary file called boot.bin. 
 ;Now we need to create a disk image and copy the boot.bin file to it. 
 ;dd if=/dev/zero of=disk.img bs=512 count=2880