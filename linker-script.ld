ENTRY(_start)
SECTIONS
{
    . = 0x40080000;
    /* Merge all text sections, since Rust - for some yet unknown reasons - creates separate sections per function */
    .text : { *(.text*) }
    .data : { *(.data) }
    .rodata : { *(.rodata*) }
    .bss : { *(.bss) }

    . = ALIGN(8);

    /* Create 1KiB space for stack, to avoid overriding read-only data */
    . = . + 0x1000;

    LD_STACK_PTR = .;
}