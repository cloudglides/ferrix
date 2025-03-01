OUTPUT_FORMAT(elf32-i386)
ENTRY(_start)

PHDRS {
    note PT_NOTE FLAGS(0); /* PVH note segment */
    text PT_LOAD FLAGS(7); /* RWE */
}

SECTIONS {
    . = 1M; /* Load address */

    /* PVH Note - MUST be first section */
    .note.pvh : ALIGN(4) {
        LONG(4)           /* namesz = 4 */
        LONG(4)           /* descsz = 4 */
        LONG(0x12)        /* type = XEN_ELFNOTE_PHYS32_ENTRY */
        BYTE(0x58) BYTE(0x65) BYTE(0x6e) BYTE(0x00) /* "Xen" + null */
        LONG(_start)      /* Entry point address */
        . = ALIGN(4);
    } :note

    /* Kernel start symbol */
    KERNEL_START = .;

    /* Text section (executable code) */
    .text : ALIGN(4K) {
        *(.text .text.*)
    } :text

    /* Read-only data */
    .rodata : ALIGN(4K) {
        *(.rodata .rodata.*)
    } :text

    /* Initialized data */
    .data : ALIGN(4K) {
        *(.data .data.*)
    } :text

    /* Uninitialized data (BSS) */
    .bss : ALIGN(4K) {
        *(.bss .bss.*)
    } :text

    /* Kernel end symbol */
    KERNEL_END = .;

    /* Discard unnecessary sections */
    /DISCARD/ : {
        *(.comment)
        *(.eh_frame)
    }
}
