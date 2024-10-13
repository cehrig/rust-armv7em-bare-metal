ENTRY(_start)


MEMORY {
    flash(rx) : ORIGIN = 0x08000000, LENGTH = 2048k
    sram(rx) : ORIGIN = 0x20000000, LENGTH = 256k
}

SECTIONS
{
    . = 0;

    /* Merge all text sections, since Rust - for some yet unknown reasons - creates separate sections per function */
    .text : {
        KEEP(*(.vector_table))
        *(.text._start)
        *(.text.*)
    } > flash

    .data : { *(.data) __data_end = .; } > sram AT > flash

    /*
    .rodata : { *(.rodata*) }
    .bss : { *(.bss COMMON) }
    */

    /DISCARD/ : { *(.comment) *(.ARM.*) }

    . = ALIGN(4);

    /* Create 1KiB space for stack, to avoid overriding read-only data */
    . = . + 0x1000;

    LD_STACK_PTR = .;
}