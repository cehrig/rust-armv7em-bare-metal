ENTRY(_start)

MEMORY {
    flash(rx) : ORIGIN = 0x08000000, LENGTH = 2048k
    sram(rx) : ORIGIN = 0x20000000, LENGTH = 128k
    sram3(rx) : ORIGIN = 0x20020000, LENGTH = 64k
}

SECTIONS
{
    . = 0;

    /* Merge all text sections, since Rust - for some yet unknown reasons - creates separate sections per function */
    .text : {
        __text_start = .;
        KEEP(*(.vector_table))
        *(.text.*)
        __text_end = .;
    } > flash

    .data : {
        __data_start = .;
        *(.data)
        __data_end = .;
    } > sram AT > flash
    .rodata : { *(.rodata*) } > sram AT > flash

    /*
    .bss : { *(.bss COMMON) }
    */

    /DISCARD/ : { *(.comment) *(.ARM.*) }

    . = ALIGN(4);

    /* Initial Stack pointer aligned to end of sram */
    . = ORIGIN(sram) + LENGTH(sram);

    LD_STACK_PTR = .;
}