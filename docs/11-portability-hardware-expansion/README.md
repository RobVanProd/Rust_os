# 11 · Portability & Hardware Expansion

This stage focuses on broadening hardware support and ensuring the OS can run on diverse architectures.

1.  **Abstract HAL Targets**
    *   Further develop and refine the Hardware Abstraction Layer (HAL) to support multiple CPU architectures.
    *   Primary targets for porting beyond the initial x86-64 implementation include:
        *   **RISC-V**: A key open ISA, suitable for various devices.
        *   **aarch64 (ARM64)**: Particularly for laptops (e.g., M-series Apple Silicon analogues) and embedded systems.
    *   Future consideration for x86 SVM (Secure Virtual Machine) extensions for advanced virtualization features.

2.  **Cross-Platform Graphics**
    *   The Wayland-based graphics protocol developed in Stage 6 is designed to be platform-agnostic.
    *   Portability efforts will focus on ensuring the `wgpu` renderer back-ends (Vulkan, Metal, DX12) function correctly on new hardware targets. The core windowing and compositing logic should remain largely unchanged.

3.  **Driver Ecosystem Growth**
    *   Encourage and facilitate the development of new drivers for a wider range of peripherals.
    *   This includes refining the driver model (Stage 2) and providing tools and documentation for third-party driver developers.

By completing this stage, the OS will be capable of running on a wider variety of hardware, increasing its reach and applicability.
