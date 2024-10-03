[BITS 64]

global _start

CODE_SEG equ 0x08
DATA_SEG equ 0x10

_start:
                                        ; Set up segment registers for 64-bit mode
    mov AX, DATA_SEG
    mov DS, AX
    mov ES, AX
    mov FS, AX
    mov GS, AX
    mov SS, AX

                                        ; Set base pointer and stack pointer to 2MB (64-bit)
    mov RBP, 0x00200000                 ; Set the base pointer (64-bit register)
    mov RSP, RBP                        ; Set the stack pointer (64-bit register)

                                        ; Keyboard controller setup (reading from port 0x92)
    in AL, 0x92                         ; Read the value of the keyboard controller
    or AL, 2
    out 0x92, AL                        ; Write the value back to the keyboard controller

                                        ; Infinite loop (halt CPU execution)
    jmp $

times 512 - ($ - $$) db 0               ; Fill to 512 bytes
