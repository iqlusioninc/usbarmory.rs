/* NOTE if you modify this file then you'll need to run the `assemble.sh` script */
  .global _start
  .section .text._start
  .type _start, %function
_start:
  /* initial stack pointer */
  ldr sp,=__stack_top__

  /* set VBAR (Vector Base Address) */
  ldr r0,=_exceptions
  mcr p15, 0, r0, c12, c0, 0

  /* jump into the Rust part of the entry point */
  b start
