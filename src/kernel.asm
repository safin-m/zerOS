[BITS 32]

global _start

CODE_SEG equ 0x08
DATA_SEG equ 0x10

_start:
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

times 512 - ($ - $$) db 0