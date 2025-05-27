# 2 · Core Kernel Services

This stage builds upon the minimal kernel by adding essential services for task management, inter-process communication, and device interaction.

1.  **Pre-emptive Scheduler**
    *   Implement a pre-emptive scheduling algorithm (e.g., MLFQ or a CFS variant).
    *   Tasks will be represented by a pinned structure (e.g., `TaskRef`) integrated with asynchronous executors.

2.  **Capability & Inter-Process Communication (IPC)**
    *   Establish a kernel global channel registry for IPC.
    *   Capabilities (`Cap`) will wrap shared, send-once, revocable channels (e.g., `Arc<Channel<T>>`), forming the basis of the security model.

3.  **Driver Model**
    *   Define a `Driver` trait and a service manager for dynamically loading and managing drivers.
    *   Enumerate essential drivers early (serial, keyboard, PCI).
    *   Design for hot-pluggable devices.
    *   Encourage user-space drivers operating behind a device server façade to enhance system stability and security.

4.  **Error Handling & Observability**
    *   Implement a `#[panic_handler]` that ties into a kernel log ring-buffer for capturing critical errors.
    *   Integrate first-class tracing events throughout the kernel for debugging and performance analysis.

By the end of this stage, the kernel will be able to manage multiple tasks, allow secure communication between them, and interact with basic hardware drivers.
