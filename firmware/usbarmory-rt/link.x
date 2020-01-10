/* Entry point of the ELF image */
ENTRY(_start);

/* # Memory regions */
MEMORY
{
  /* TODO figure out where u-boot places its own memory */
  /* it may not be possible to interactively load a program in that memory region */

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

__stack_top__ = ORIGIN(OCRAM) + LENGTH(OCRAM);

/* # Linker sections */
SECTIONS
{
  /* ## Standard ELF sections */
  .text :
  {
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
