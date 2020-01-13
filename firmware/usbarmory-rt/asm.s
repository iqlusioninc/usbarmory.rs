/* NOTE if you modify this file then you'll need to run the `assemble.sh` script */
  .global _start
  .section .text._start
  .type _start, %function
_start:
  /* initial stack pointer */
  ldr sp,=__stack_top__

  /* set VBAR (Vector Base Address) */
  movw r0, #:lower16:_exceptions
  movt r0, #:upper16:_exceptions
  mcr p15, 0, r0, c12, c0, 0

  /* jump into the Rust part of the entry point */
  b start

  .global _exceptions
  .section .text._exceptions
  .type _exceptions, %function
_exceptions:
  ldr pc, =DefaultHandler       /* 0x00: Reset, but this will never be called */
  ldr pc, =UndefinedInstruction /* 0x04; Undefined Instruction */
  ldr pc, =SupervisorCall       /* 0x08: Supervisor Call */
  ldr pc, =PrefetchAbort        /* 0x0C: Prefetch Abort */
  ldr pc, =DataAbort            /* 0x10: Data Abort */
  ldr pc, =HypervisorCall       /* 0x14: Hypervisor Trap */
  ldr pc, =IRQ                  /* 0x18: IRQ interrupt */
  ldr pc, =FIQ                  /* 0x1C: FIQ interrupt */
