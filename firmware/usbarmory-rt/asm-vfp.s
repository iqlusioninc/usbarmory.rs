/* NOTE if you modify this file then you'll need to run the `assemble.sh` script */
  .global _start
  .section .text._start
  .type _start, %function
_start:
  /* initial stack pointer */
  ldr sp,=__stack_top__

  /* enable VFP support */
  /* see section 2.1.2 of DDI 0463F */
  mov  r0, #(0xf << 20)
  mcr  p15, 0, r0, c1, c0, 2
  mov  r0, #(1 << 30)
  vmsr fpexc, r0

  /* set VBAR (Vector Base Address) */
  movw r0, #:lower16:_exceptions
  movt r0, #:upper16:_exceptions
  mcr p15, 0, r0, c12, c0, 0

  /* jump into the Rust part of the entry point */
  b start
