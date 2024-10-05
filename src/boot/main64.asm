; This assembly code is part of an OS bootloader that transitions from 32-bit protected mode to 64-bit long mode.
; It sets up the necessary segment registers and then jumps to the main kernel entry point.

global long_mode_start
extern kernel_main

section .text
bits 64

; The entry point for 64-bit long mode
long_mode_start:
    ; Clear the segment registers to ensure they are set to a known state.
    ; This is important because in long mode, segment registers are not used in the same way as in 32-bit mode.
    ; They should be zeroed out to avoid any unexpected behavior.
    mov AX, 0
    mov DS, AX  ; Data Segment
    mov ES, AX  ; Extra Segment
    mov FS, AX  ; FS Segment
    mov GS, AX  ; GS Segment
    mov SS, AX  ; Stack Segment

    ; Call the main kernel entry point.
    ; This is where the main kernel code will start executing.
    ; The kernel_main function is expected to be defined elsewhere in the kernel code.
    call kernel_main