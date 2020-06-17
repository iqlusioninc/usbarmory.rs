# `cortex-a-rtfm`

[Real Time for The Masses][rtfm] (RTFM) for Cortex-A processors that use ARM's
GIC (General Interrupt Controller) as their interrupt controller

[rtfm]: https://rtfm.rs

This port supports the following functionality

- Resources: late initialization and sharing (`lock` API)
- Software tasks (`#[task]`) & message passing (`spawn` API)
- Hardware tasks (`#[task(binds = ..)`)

This port does not yet support the following functionality

- Timer queue (`schedule` API)

This port has hard-coded the following parameters but could be made more
generic:

- single-core SoC
- GIC version 2.0
- 32 priority levels (Cortex-A7 specific)
- the base address of the GIC peripheral (imx6ul specific)


