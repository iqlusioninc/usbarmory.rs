/* Entry point of the ELF image */
ENTRY(_start);

/* # Memory regions */
MEMORY
{
  /* On-chip RAM */
  OCRAM : ORIGIN = 0x00900000, LENGTH = 128K

  /* DDR3 RAM */
  /* the first 1024B of padding is required for booting from eMMC/uSD */
  /* the second 1024B of padding is space reserved for the IVT, Boot Data and DCD  */
  /* NOTE if you modify the size of the second section in `host/image` then
     you'll need to modify the ORIGIN */
  DRAM : ORIGIN = 0x80000800, LENGTH = 512M
}

/* Use the default exception handler to handle all exceptions that have not been set by the user */
PROVIDE(UndefinedInstruction = DefaultHandler);
PROVIDE(SupervisorCall = DefaultHandler);
PROVIDE(PrefetchAbort = DefaultHandler);
PROVIDE(DataAbort = DefaultHandler);
PROVIDE(HypervisorCall = DefaultHandler);
PROVIDE(IRQ = DefaultHandler);
PROVIDE(FIQ = DefaultHandler);

/* Same thing with unset interrupts */
PROVIDE(SGI0 = DefaultHandler);
PROVIDE(SGI1 = DefaultHandler);
PROVIDE(SGI2 = DefaultHandler);
PROVIDE(SGI3 = DefaultHandler);
PROVIDE(SGI4 = DefaultHandler);
PROVIDE(SGI5 = DefaultHandler);
PROVIDE(SGI6 = DefaultHandler);
PROVIDE(SGI7 = DefaultHandler);
PROVIDE(SGI8 = DefaultHandler);
PROVIDE(SGI9 = DefaultHandler);
PROVIDE(SGI10 = DefaultHandler);
PROVIDE(SGI11 = DefaultHandler);
PROVIDE(SGI12 = DefaultHandler);
PROVIDE(SGI13 = DefaultHandler);
PROVIDE(SGI14 = DefaultHandler);
PROVIDE(SGI15 = DefaultHandler);

INCLUDE interrupts.x

/* Make the linker exhaustively search these symbols, otherwise they may be ignored even if provided */
EXTERN(_exceptions);

/* Top of the stack */
PROVIDE(__stack_top__ = ORIGIN(OCRAM) + LENGTH(OCRAM));

/* Where to place things that are not the stack in RAM */
PROVIDE(__ram_start__ = ORIGIN(OCRAM));

/* # Linker sections */
SECTIONS
{
  /* ## Standard ELF sections */
  .text :
  {
    /* put the entry point first to make the objdump easier to read */
    *(.text._start);
    *(.text.start);

    /* the exception vector has an alignment requirement */
    . = ALIGN(32);
    KEEP(*(.text._exceptions));

    *(.text .text.*);
  } > DRAM

  .rodata :
  {
    *(.rodata .rodata.*);
    /* align the end of this section so `_sidata` ends up aligned as well */
    . = ALIGN(4);
  } > DRAM

  .data __ram_start__ :
  {
    . = ALIGN(4);
    _sdata = .;
    *(.data .data.*);
    . = ALIGN(4);
    _edata = .;
  } > OCRAM AT>DRAM

  _sidata = LOADADDR(.data);

  .bss ADDR(.data) + SIZEOF(.data) (NOLOAD) :
  {
    . = ALIGN(4);
    _sbss = .;
    *(.bss .bss.*);
    . = ALIGN(4);
    _ebss = .;
  } > OCRAM

  .uninit ADDR(.uninit) + SIZEOF(.uninit) (NOLOAD) :
  {
    *(.uninit.*);
  } > OCRAM

  /* ## Discarded sections */
  /DISCARD/ :
  {
    /* Information required for unwinding that's used by Rust applications */
    *(.ARM.exidx);
    *(.ARM.exidx.*);
    *(.ARM.extab.*);
  }
}

/* alignment requirement */
ASSERT(_exceptions % 32 == 0, "exception vector is not 32-byte aligned");
ASSERT(_sdata % 4 == 0 && _edata % 4 == 0 && _sidata % 4 == 0, "`.data` is not 4-byte aligned");
ASSERT(_sbss % 4 == 0 && _ebss % 4 == 0, "`.bss` is not 4-byte aligned");
