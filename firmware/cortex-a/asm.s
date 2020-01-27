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


  .global __enable_fiq
  .section .text.__enable_fiq
__enable_fiq:
  cpsie f
  bx lr


  .global __disable_fiq
  .section .text.__disable_fiq
__disable_fiq:
  cpsid f
  bx lr


  .global __enable_irq
  .section .text.__enable_irq
__enable_irq:
  cpsie i
  bx lr


  .global __disable_irq
  .section .text.__disable_irq
__disable_irq:
  cpsid i
  bx lr


  .global __dmb
  .section .text.__dmb
__dmb:
  dmb sy
  bx lr


  .global __dsb
  .section .text.__dsb
__dsb:
  dsb sy
  bx lr


  .global __isb
  .section .text.__isb
__isb:
  isb
  bx lr

  .global __wfi
  .section .text.__wfi
__wfi:
  wfi
  bx lr

  .global __udf
  .section .text.__udf
__udf:
  udf
