/* NOTE if you modify this file then you'll need to run the `assemble.sh` script */
  .global _start
  .section .text._start
  .type _start, %function
_start:
  /* initial stack pointer */
  ldr sp,=__stack_top__

  /* disable the caches */
  mrc 15, 0, r0, cr1, cr0, 0
  bic r0, r0, #4
  mcr 15, 0, r0, cr1, cr0, 0
  isb sy

  /* set VBAR (Vector Base Address) */
  movw r0, #:lower16:_exceptions
  movt r0, #:upper16:_exceptions
  mcr p15, 0, r0, c12, c0, 0

  /* jump into the Rust part of the entry point */
  b start
