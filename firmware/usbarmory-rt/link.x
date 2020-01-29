/* Entry point of the ELF image */
ENTRY(_start);

/* # Memory regions */
MEMORY
{
  /* NOTE u-boot places its memory at the start of DRAM. When interactively */
  /* loading programs u-boot will automatically place them after the memory */
  /* it's using */

  /* On-chip RAM */
  OCRAM : ORIGIN = 0x00900000, LENGTH = 128K

  /* Secure RAM */
  CAAM : ORIGIN = 0x00100000, LENGTH = 32K

  /* DDR3 RAM */
  DRAM : ORIGIN = 0x80000000, LENGTH = 512M
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
  .text __ram_start__ :
  {
    /* put the entry point first to make the objdump easier to read */
    *(.text._start);
    *(.text.start);

    /* the exception vector has an alignment requirement */
    . = ALIGN(32);
    KEEP(*(.text._exceptions));

    *(.text .text.*);
  } > OCRAM

  .rodata ADDR(.text) + SIZEOF(.text) :
  {
    *(.rodata .rodata.*);
  } > OCRAM

  .data ADDR(.rodata) + SIZEOF(.rodata) :
  {
    *(.data .data.*);
  } > OCRAM

  .bss ADDR(.data) + SIZEOF(.data) :
  {
    *(.bss .bss.*);
  } > OCRAM

  /* ## Discarded sections */
  /DISCARD/ :
  {
    /* We are not using a debugger so we discard the DWARF sections
       This makes the ELF file much smaller, which makes transfers over the
       slow serial interface much faster */
    *(.debug_*);

    /* Information required for unwinding that's used by Rust applications */
    *(.ARM.exidx);
    *(.ARM.exidx.*);
    *(.ARM.extab.*);
  }
}

/* alignment requirement */
ASSERT(_exceptions % 32 == 0, "exception vector is not 32-byte aligned");
