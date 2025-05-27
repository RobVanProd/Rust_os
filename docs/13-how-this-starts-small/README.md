# How This “Starts Small”

This section outlines the initial, incremental milestones to demonstrate core functionality and validate the architecture progressively. Each step builds a foundation for the next, ensuring a manageable and testable development process.

1.  **Boot + Serial Hello World**
    *   **Goal**: Validates the bootstrap process (Stage 1).
    *   **Outcome**: The kernel successfully boots, initializes basic hardware (CPU, timers, serial UART), and prints a "Hello, World!" message to the serial console.
    *   **Focus**: `bootloader` crate integration, basic HAL setup, serial output.

2.  **Memory + Tasks → Run Two Fibres Blinking LEDs/Logging**
    *   **Goal**: Validates physical/virtual memory setup (Stage 1) and the pre-emptive scheduler (Stage 2).
    *   **Outcome**: The kernel can create and schedule at least two independent tasks (or "fibres") that perform simple, observable actions (e.g., conceptually blinking LEDs by printing alternating messages to the serial log, or toggling GPIO pins if on suitable hardware).
    *   **Focus**: Frame allocator, page mapping, task creation, context switching, basic IPC for coordination if needed.

3.  **User Process → Load a Unix-style “init” Echo Server**
    *   **Goal**: Validates the ELF loader and basic IPC/service layers (Stage 3).
    *   **Outcome**: The kernel can load a simple ELF executable from a known location (e.g., initramfs or simple filesystem). This "init" process acts as an echo server: it waits for messages via an IPC channel and sends them back. Another process (or the kernel itself) can test this.
    *   **Focus**: ELF parsing, user address space setup, capability hand-off for IPC, minimal `libc` for the user process.

4.  **Framebuffer Text Console → Simple VGA -> Later Swapped by Wayland Compositor**
    *   **Goal**: Validates basic graphics output (Stage 6, initial part) before the full Wayland stack.
    *   **Outcome**: The kernel (or a simple user-space service) can write text to a VGA text mode or a simple framebuffer console. This provides a visual output beyond serial logging.
    *   **Focus**: PCI BAR access for framebuffer, font rendering (simple bitmap font), cursor management. This will be superseded by the full Wayland compositor and `wgpu` rendering later but serves as an early visual milestone.

5.  **Package Manager & GUI Shell → After Kernel/Userland Stabilise**
    *   **Goal**: Demonstrates higher-level application management and user interaction (Stages 7 & 8).
    *   **Outcome**: Basic package management allows installing a simple application. A minimal GUI shell (even if just a window with a button) can be launched.
    *   **Focus**: This comes much later, once the core kernel, user-space services, and graphics stack are sufficiently mature.

6.  **AI Daemon → Bolt On Once IPC/Perf Are Trustworthy**
    *   **Goal**: Validates the AI integration strategy (Stage 9).
    *   **Outcome**: A simple AI model can be loaded and an inference request can be processed via the system AI bus.
    *   **Focus**: This is also a later-stage goal, dependent on stable IPC, resource management, and potentially GPU access via `wgpu`.

Each milestone introduces one new subsystem while tightening tests, documentation, and safety guarantees established earlier. The architecture therefore remains lean, auditable, and already “AI-aware” long before heavy models ship.
