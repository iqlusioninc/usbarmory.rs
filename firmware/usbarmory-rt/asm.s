/* NOTE if you modify this file then you'll need to run the `assemble.sh` script */
  .global _start
  .section .text._start
  .type _start, %function
_start:
  /* TODO initialize registers */
  /* TODO conditionally enable FPU */
  ldr sp,=__stack_top__
  b start
