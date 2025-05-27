# 4 · Storage & Filesystems

This stage introduces the ability to interact with storage devices and manage files.

1.  **Virtual File System (VFS) Core**
    *   Implement a VFS core layer with support for pluggable back-ends.
    *   Initially, implement a FAT-32 filesystem driver within the kernel for simplicity and to support common boot media.
    *   Plan to move towards a journaling filesystem (e.g., ext4, or a custom Rust-based one) running in user space for enhanced robustness and security.

2.  **Path-Based Capabilities**
    *   The `open` operation (or its equivalent) will return file capabilities.
    *   These capabilities will have read, write, and execute flags baked into them, integrating filesystem access with the system's capability-based security model.

At the end of this stage, the OS will be able to mount filesystems, and processes will be able to access files based on granted capabilities.
