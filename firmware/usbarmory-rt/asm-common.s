  .global _exceptions
  .section .text._exceptions
  .type _exceptions, %function
_exceptions:
  ldr pc, =DefaultHandler        /* 0x00: Reset, but this will never be called */
  ldr pc, =_UndefinedInstruction /* 0x04; Undefined Instruction */
  ldr pc, =SupervisorCall        /* 0x08: Supervisor Call */
  ldr pc, =_PrefetchAbort        /* 0x0C: Prefetch Abort */
  ldr pc, =DataAbort             /* 0x10: Data Abort */
  ldr pc, =_HypervisorCall       /* 0x14: Hypervisor Trap */
  ldr pc, =_IRQ                  /* 0x18: IRQ interrupt */
  ldr pc, =_FIQ                  /* 0x1C: FIQ interrupt */


  /* TODO on `eabihf` we need to stack the VFP registers */
  .global _IRQ
  .section .text._IRQ
  .type _IRQ, %function
_IRQ:
  /* NOTE at this point we are in IRQ mode (_irq) */
  /* NOTE some registers are banked; i.e. there's a copy for each mode; we'll
     use suffix to distinguish the banked values; e.g. LR_irq vs LR_svc */

  /* compute return address; see tables B1-6 & B1-7 of DDI 0406C.c */
  sub lr, lr, #4                ; /* NOTE this is LR_irq */

  /* (A) push LR_irq & SPSR_irq onto the Supervisor mode (_svc) stack */
  srsdb sp!, #19

  /* return to Supervisor mode; see sec. B1.3.1 of DDI 0406C.c */
  cps #19

  /* (B) stash scratch registers; see sec. 6.1.1 of AAPCS 2019Q1.1 */
  /* AAPCS says that IRQ (C ABI) is free to use these scratch registers */
  push {r0-r3, ip}

  /* test alignment of the stack */
  and r0, sp, #4

  /* (C) 8-byte align the stack; required by AAPCS */
  sub sp, sp, r0

  /* (D) stash the stack adjustment and LR_svc */
  /* (the next `bl` instruction will overwrite LR_svc) */
  /* (this LR_svc value is the linker register of the preempted context) */
  push {r0, lr}

  /* jump into the Rust part of this exception handler */
  bl IRQ

  /* retrieve the stack adjustment and LR_svc; pairs with (D) */
  pop {r0, lr}

  /* undo the stack adjustment; pairs with (C) */
  add sp, sp, r0

  /* restore scratch registers; pairs with (B) */
  pop {r0-r3, ip}

  /* return from exception; pairs with (A) */
  /* (this restores CPSR (<- SPRSR_irq) and jumps to LR_irq)) */
  rfeia sp!

/* we have not set a stack pointer for each mode; instead we'll have all modes
  use the Supervisor mode (_svc) stack */
/* the use of `b` (branch) assumes that the called subroutine will never return */

/* _hyp mode */
  .global _HypervisorCall
  .section .text._HypervisorCall
  .type _HypervisorCall, %function
_HypervisorCall:
  cps #19
  b HypervisorCall


/* _abt mode */
  .global _PrefetchAbort
  .section .text._PrefetchAbort
  .type _PrefetchAbort, %function
_PrefetchAbort:
  cps #19
  b PrefetchAbort


  .section .rodata.DataAbortMsg
DataAbortMsg:
  .ascii "\ndata abort exception (it could indicate a stack overflow)\n"
DataAbortMsgLen = . - DataAbortMsg

/* NOTE the Rust code for this assembly can be found in `src/lib.rs` */
/* _abt mode */
  .global DataAbort
  .section .text.DataAbort
  .type DataAbort, %function
DataAbort:
  /* Serial::write_all */
  movw  ip, #:lower16:DataAbortMsg
  movw  r0, #0x8040
  movt  ip, #:upper16:DataAbortMsg
  movt  r0, #0x021e
  mov   r2, ip
1:
  ldrb  r3, [r2]
2:
  ldr   r1, [r0, #0x54]
  tst   r1, #0x2000
  beq   2b
  add   r2, r2, #1
  add   r1, ip, #DataAbortMsgLen
  cmp   r2, r1
  str   r3, [r0]
  bne   1b

  /* Serial::flush */
3:
  ldr   r1, [r0, #0x58]
  tst   r1, #8
  beq   3b

  /* usbarmory::reset */
  movw  r0, 0xc000
  movt  r0, 0x20b
  ldrh  r1, [r0]
  bic   r1, r1, #16
  strh  r1, [r0]
4:
  b     4b


/* _und mode */
  .global _UndefinedInstruction
  .section .text._UndefinedInstruction
  .type _UndefinedInstruction, %function
_UndefinedInstruction:
  cps #19
  b UndefinedInstruction


/* _fiq mode */
  .global _FIQ
  .section .text._FIQ
  .type _FIQ, %function
_FIQ:
  cps #19
  b FIQ
