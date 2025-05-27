# 6 · Graphics & Compositor

This stage focuses on building the foundational layers for a graphical user interface.

1.  **Framebuffer Service**
    *   Implement a basic framebuffer service, allowing direct drawing to a linear framebuffer.
    *   Access to the framebuffer will typically be achieved via PCI BAR (Base Address Register) mapping for VGA-compatible graphics hardware or virtualized display.

2.  **Wayland-like Protocol**
    *   Design and implement a Wayland-like protocol for communication between clients (applications) and the compositor.
    *   Utilize components from libraries like Smithay to manage:
        *   Object life-cycle (surfaces, buffers, etc.)
        *   Input handling (keyboard, mouse, touch)
        *   Output management (display configuration)

3.  **`wgpu` Renderer**
    *   Integrate `wgpu` as the rendering abstraction layer.
    *   `wgpu` will provide an API over underlying graphics libraries like Vulkan, Metal, or DirectX, allowing for hardware acceleration as it becomes available on different platforms.

4.  **Window Manager Layer**
    *   Develop a basic window manager that runs on top of the compositor.
    *   This layer will handle:
        *   Window decorations (borders, title bars)
        *   Tiling or stacking window management logic
        *   Basic compositor effects (e.g., transparency, if feasible with `wgpu`)

By the end of this stage, the OS will have a functional graphics stack capable of rendering application windows and managing user input, paving the way for a full desktop environment.
