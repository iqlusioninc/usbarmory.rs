/* Entry point of the ELF image */
ENTRY(_start);

/* # Memory regions */
MEMORY
{
  /* NOTE u-boot places its memory at the start of DRAM. When interactively */
  /* loading programs u-boot will automatically place them after the memory */
  /* it's using */

  /* On-chip RAM */
  /* NOTE OCRAM starts at 0x00900000 and its physical size is 128 KB but the
     first 28 KB are reserved and the following 68 KB are free according to
     section 8.4.1 of the RM */
  OCRAM : ORIGIN = 0x00907000, LENGTH = 68K

  /* Secure RAM */
  CAAM : ORIGIN = 0x00100000, LENGTH = 32K

  /* DDR3 RAM */
  DRAM : ORIGIN = 0x80000000, LENGTH = 512M
}

/* Initial Stack Pointer */
__stack_top__ = ORIGIN(OCRAM) + LENGTH(OCRAM);

/* Use the default exception handler to handle all exceptions that have not been set by the user */
PROVIDE(UndefinedInstruction = DefaultHandler);
PROVIDE(SupervisorCall = DefaultHandler);
PROVIDE(PrefetchAbort = DefaultHandler);
PROVIDE(DataAbort = DefaultHandler);
PROVIDE(HypervisorCall = DefaultHandler);
PROVIDE(IRQ = DefaultHandler);
PROVIDE(FIQ = DefaultHandler);

/* Make the linker exhaustively search these symbols, otherwise they may be ignored even if provided */
EXTERN(_exceptions);

/* # Linker sections */
SECTIONS
{
  /* ## Standard ELF sections */
  .text :
  {
    /* must go first due to alignment requirements */
    KEEP(*(.text._exceptions));

    /* NOTE order the entry point to make the objdump easier to read */
    *(.text._start);
    *(.text.start);

    *(.text .text.*);
  } > OCRAM

  .rodata :
  {
    *(.rodata .rodata.*);
  } > OCRAM

  .data :
  {
    *(.data .data.*);
  } > OCRAM

  .bss :
  {
    *(.bss .bss.*);
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
ASSERT(ADDR(.text) % 32 == 0, "exception vector is not 32-bit aligned");
