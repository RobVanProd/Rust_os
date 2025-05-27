# 3 · Process, ELF & Service Layer

This stage focuses on enabling the execution of user-space programs and establishing a basic service supervision system.

1.  **Executable Loader**
    *   Implement functionality to parse 64-bit Position-Independent Executable (PIE) ELF files.
    *   Relocate ELF segments into a user address space.
    *   Hand off capabilities for standard streams (stdin, stdout, stderr) to the newly loaded process.

2.  **Minimal `libc` Subset**
    *   Adopt a subset of `rustix` or a similar library to expose POSIX-like system calls to user programs.
    *   These calls will be implemented over the kernel's message-passing IPC system.

3.  **Init / Service Supervisor**
    *   Develop an initial process or service supervisor.
    *   Service configurations will be described using TOML unit descriptors.
    *   These descriptors will declare restart policies, required capability sets, and environment variables for each service.

Completion of this stage will allow the OS to load and run simple user-space executables and manage basic system services.
