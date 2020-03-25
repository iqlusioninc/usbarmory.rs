/* NOTE if you modify this file then you'll need to run the `assemble.sh` script */
  .global _start
  .section .start._start
  .type _start, %function
_start:
  /* initial stack pointer */
  ldr sp,=__stack_top__

  /* copy .text from DRAM into OCRAM */
  ldr r0,=_stext
  ldr r1,=_etext
  ldr r2,=_sitext
1:
  cmp  r0, r1
  bcs  2f
  ldr  r3, [r2], #4
  str  r3, [r0], #4
  b    1b

  /* copy .rodata from DRAM into OCRAM */
2:
  ldr r0,=_srodata
  ldr r1,=_erodata
  ldr r2,=_sirodata
3:
  cmp  r0, r1
  bcs  4f
  ldr  r3, [r2], #4
  str  r3, [r0], #4
  b    3b

  /* set VBAR (Vector Base Address) */
4:
  ldr r0,=_exceptions
  mcr p15, 0, r0, c12, c0, 0

  /* jump into the Rust part of the entry point */
  ldr r0, =start
  bx  r0
