# 1 · Bootstrap & Minimal Kernel

This stage focuses on getting the operating system to boot and establishing the most basic hardware interactions.

1.  **Boot path**
    *   Use `bootloader` crate to enter 64-bit long mode.
    *   Map higher-half kernel.
    *   Set up stack.
    *   Set up Interrupt Descriptor Table (IDT).

2.  **Hardware Abstraction Layer (HAL)**
    *   CPU initialization (targeting x86-64 initially).
    *   Define abstract traits for CPU features to support later porting to RISC-V/ARM.
    *   Implement basic timer support (HPET/APIC).
    *   Implement UART/serial logging for early diagnostics.

3.  **Physical & Virtual Memory**
    *   Implement a buddy allocator for physical memory frames.
    *   Create a safe wrapper for frame references (e.g., `FrameRef`).
    *   Define an abstract `Mapper` trait for paging operations.
    *   Userspace processes will eventually receive a per-task `MemoryCtx` for memory management.

Successfully completing this stage means the kernel can boot, print to a serial console, and has foundational memory management structures in place.
