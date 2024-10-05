global long_mode_start
extern kernel_main

section .text
bits 64

long_mode_start:
    mov AX, 0
    mov DS, AX
    mov ES, AX
    mov FS, AX
    mov GS, AX
    mov SS, AX

    call kernel_main