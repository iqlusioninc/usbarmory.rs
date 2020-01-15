  .global _exceptions
  .section .text._exceptions
  .type _exceptions, %function
_exceptions:
  ldr pc, =DefaultHandler        /* 0x00: Reset, but this will never be called */
  ldr pc, =_UndefinedInstruction /* 0x04; Undefined Instruction */
  ldr pc, =SupervisorCall        /* 0x08: Supervisor Call */
  ldr pc, =_PrefetchAbort        /* 0x0C: Prefetch Abort */
  ldr pc, =_DataAbort            /* 0x10: Data Abort */
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


/* _abt mode */
  .global _DataAbort
  .section .text._DataAbort
  .type _DataAbort, %function
_DataAbort:
  cps #19
  b DataAbort


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
