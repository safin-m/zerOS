section .multiboot_header

header_start:
    ; Multiboot2 magic number
    dd 0xe85250D6

    ; Reserved, must be 0
    dd 0

    ; Length of the multiboot header
    dd header_end - header_start

    ; Checksum of the multiboot header
    ; Ensures that the sum of all the 32-bit values in the header is 0
    dd 0x100000000 - (0xe85250D6 + 0 + (header_end - header_start))

    ; Reserved, must be 0
    dw 0

    ; Reserved, must be 0
    dw 0

    ; Architecture type (8 indicates x86)
    dd 8

header_end: