/* 
 * Specifies the entry point of the program.
 * The entry point is the address where the execution starts.
 * In this case, it is set to the symbol `start`.
 */
ENTRY(start)

SECTIONS
{
    /* 
     * Sets the location counter to 1 megabyte (1M).
     * The location counter is a variable that holds the current memory address.
     * Setting it to 1M means that the following sections will be placed starting at the 1MB address.
     * This is a common practice in OS development to leave the lower memory addresses for the BIOS and other system components.
     */
    . = 1M;

    /* 
     * Defines a section named `.boot` to store the multiboot header.
     */
    .boot :
    {
        /* 
         * Ensures that the `.multiboot_header` section is not removed by the linker, even if it is not referenced.
         * This is crucial for multiboot-compliant boot loaders to find the multiboot header.
         */
        KEEP(*(.multiboot_header))
    }

    /* 
     * Defines a section named `.text` to store the executable code of the program.
     */
    .text :
    {
        /* 
         * Includes all input sections named `.text` from the object files.
         * This is where the actual code of the kernel or other executable components will be placed.
         */
        *(.text)
    }
}