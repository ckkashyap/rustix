ENTRY(EntryPoint)

/*KERNEL_VMA = 0x100000;*/
KERNEL_VMA = 0xFFFFFFFF80000000;

SECTIONS
{

       . = 0x100000;

       .boot :
       {
               *(.mbhdr)
               _loadStart = .;
               *(.boot)
               . = ALIGN(4096);
               Pml4 = .;
               . += 0x1000;
               Pdpt = .;
               . += 0x1000;
               Pd = .;
               . += 0x1000;
               . += 0x8000;
               Stack = .;
       }

    . += KERNEL_VMA;

    .text : AT(ADDR(.text) - KERNEL_VMA)
    {
        _code = .;
        *(.text)
        *(.rodata*)
        . = ALIGN(4096);
    }

   .data : AT(ADDR(.data) - KERNEL_VMA)
   {
        _data = .;
        *(.data)
        . = ALIGN(4096);
   }

   .eh_frame : AT(ADDR(.eh_frame) - KERNEL_VMA)
   {
       _ehframe = .;
       *(.eh_frame)
        . = ALIGN(4096);
   }

	_loadEnd = . - KERNEL_VMA;

   .bss : AT(ADDR(.bss) - KERNEL_VMA)
   {
       _bss = .;
       *(.bss)

       /*
        * You usually need to include generated COMMON symbols
        * under kernel BSS section or use gcc's -fno-common
        */

        *(COMMON)
       . = ALIGN(4096);
   }

	_bssEnd = . - KERNEL_VMA;
   _end = .;

   /DISCARD/ :
   {
        *(.comment)
   }
}
