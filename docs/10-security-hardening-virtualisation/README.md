# 10 · Security Hardening & Virtualisation

This stage focuses on enhancing the overall security posture of the operating system and introducing capabilities for running virtualized environments.

1.  **Memory Safety Reinforcement**
    *   Strict adherence to memory-safe Rust (`#![forbid(unsafe_code)]`) throughout the majority of the codebase.
    *   `unsafe` code blocks will be permitted only within the Hardware Abstraction Layer (HAL) and rigorously audited.

2.  **Service Sandboxing**
    *   Sandbox each user-land service using advanced techniques:
        *   **Capability Filtering**: Restrict the set of capabilities available to each service to the minimum required for its operation.
        *   **Syscall Masks**: Implement seccomp-like syscall filtering to further limit the kernel attack surface from user-space processes. This will be adapted to the OS's message-passing/capability-based syscall equivalent.

3.  **Virtual Machine Monitor (VMM)**
    *   Integrate or develop a Virtual Machine Monitor (hypervisor).
    *   Initially, this could leverage KVM (Kernel-based Virtual Machine) if running on Linux for development, or aim for a custom Rust-based VMM (e.g., using `rust-vmm` components) for the native OS.
    *   The VMM will be used to host legacy guests or other operating systems in isolated environments.

Completion of this stage will significantly enhance the OS's security through strict memory safety, comprehensive sandboxing, and the ability to run potentially untrusted code in virtual machines.
