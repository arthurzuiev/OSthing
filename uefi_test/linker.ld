ENTRY(efi_main)

SECTIONS
{
    . = 0x100000;         /* start address in memory */

    .text : { *(.text*) } /* all code goes here */
    .data : { *(.data*) } /* all initialized variables */
    .bss  : { *(.bss*) }  /* all uninitialized variables */
}
