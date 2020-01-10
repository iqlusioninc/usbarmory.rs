/* NOTE if you modify this file then you'll need to run the `assemble.sh` script */
  .global __nop
  .section .text.__nop
__nop:
  bx lr

  .global __delay
  .section .text.__delay
__delay:
  lsr  r0, r0, #2
  cmp  r0, #0
  bxeq lr
1:
  subs r0, r0, #1
  nop
  nop
  bne  1b
  bx lr
