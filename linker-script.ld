ENTRY(_start)
SECTIONS
{
    . = 0x40080000;
    .text : { *(.text*) }
    .data : { *(.data) }
    .rodata : { *(.rodata) }
    .bss : { *(.bss) }

    . = ALIGN(8);
    LD_STACK_PTR = .;
}