# Rust-Native GUI Operating System — Iterative Architecture & Development Roadmap

(No time-lines are included; each stage logically builds on the last.)

---

## 0 · Vision & Core Principles

| Goal                               | Design Choice                                                                                                | Rationale                                                                                                     |
| ---------------------------------- | ------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------- |
| Memory-safe, micro-/hybrid-kernel  | All internal code in safe Rust with `#![no_std]`; kernel < 50 k SLOC; drivers ideally in user space          | Minimises whole-system exploit surface while retaining performance (see Theseus OS)                             |
| Capability-based security          | Kernel hands out cap tokens (send-once, shareable, revocable) instead of broad syscalls                        | Lets AI sub-services run safely with elevated privileges only where needed                                    |
| Message-passing first              | Zero-copy channels between tasks; asynchronous (async/await-friendly)                                        | Plays well with Rust futures and later distributed AI daemons                                                 |
| AI-ready substrate                 | Standard “AI bus” protocol baked into IPC layer; dynamic model registry                                      | Allows drop-in inference back-ends (Candle, Burn, ONNX-RT) without kernel patches                             |
| Wayland-style graphics stack       | Compositor in user space, protocol built with Smithay blocks                                                 | Modern, departs from X11 complexity, written in Rust                                                          |
| Modern GUI tool-kit                | Widget library based on iced/libcosmic plus wgpu renderer                                                    | Proven in Pop!_OS COSMIC; easily themed; GPU-accelerated                                                      |

---

## 1 · Bootstrap & Minimal Kernel

1.  **Boot path**
    Use `bootloader` crate to enter 64-bit long mode, map higher-half kernel, set up stack and IDT.
2.  **Hardware Abstraction Layer (HAL)**
    CPU init (x86-64 first; abstract traits for later RISC-V/ARM).
    Basic timers (HPET/APIC) + UART/serial logging.
3.  **Physical & Virtual Memory**
    Buddy allocator for frames → safe wrapper (`FrameRef`); abstract `Mapper` trait for paging.
    Userspace processes will receive per-task `MemoryCtx`.

---

## 2 · Core Kernel Services

1.  **Pre-emptive Scheduler**
    MLFQ or CFS variant; tasks represented by pinned `TaskRef` with async executors.
2.  **Capability & IPC**
    Kernel global channel registry; each `Cap` wraps `Arc<Channel<T>>`.
3.  **Driver Model**
    `Driver` trait + service manager; serial/keyboard/PCI enumerated early; hot-pluggable.
    Encourage user-space drivers behind a device server façade.
4.  **Error Handling & Observability**
    `#[panic_handler]` ties into kernel log ring-buffer; first-class tracing events.

---

## 3 · Process, ELF & Service Layer

1.  **Executable Loader**
    Parse 64-bit PIE ELF, relocate into user address space, hand off caps for std-streams.
2.  **Minimal libc**
    Adopt `rustix` subset to expose POSIX-like calls over message passing.
3.  **Init / Service Supervisor**
    TOML unit descriptors declare restart policy, cap set, environment.

---

## 4 · Storage & Filesystems

1.  **VFS Core** with pluggable back-ends (initially FAT-32 in kernel for simplicity, then move to journaling fs in user space).
2.  **Path-Based Capabilities** — `open` returns file caps (read/write/exec flags baked in).

---

## 5 · Networking

1.  **Net Daemon** in user land: ethernet driver → virt queue → `smoltcp`-based TCP/IP stack.
2.  **IPC bridge** exposes async sockets to apps.

---

## 6 · Graphics & Compositor

1.  **Framebuffer Service** (linear frame-buffer via PCI BAR).
2.  **Wayland-like Protocol** using Smithay for object life-cycle, input handling, output management.
3.  **`wgpu` Renderer** abstracts Vulkan/Metal/DX as hardware becomes available.
4.  **Window Manager Layer** — decoration, tiling/stacking logic, compositor effects.

---

## 7 · GUI Toolkit & Desktop Shell

1.  **`libcosmic`/`iced` Widgets**: buttons, lists, canvas → compiled to native code.
2.  **Scene Graph** with retained-mode layout engine (Flex/GTK-like).
3.  **Theme & Accessibility Engine** — CSS-like style sheets, high-contrast pipeline.

---

## 8 · Package & Update System

1.  **Immutable Base + Overlay**: root image signed; user apps as OCI-style bundles.
2.  **`cargo-pkg` Front-End**: install/uninstall apps; verify signatures; delta updates.

---

## 9 · AI Runtime Integration

1.  **Model Store Service**
    Keeps on-disk ONNX/gguf/candle checkpoint bundles; versioned, signed.
2.  **Inference Daemon(s)**
    Pluggable back-ends: candle, burn, `tch` wrapper; GPU via `wgpu-core` compute.
3.  **System AI Bus**
    Publish-subscribe over IPC (`ai/topic/*`); any app can request embeddings, chat completions, vision classification, etc.
4.  **Privacy & Quotas**
    Caps limit model size, GPU time, and data access per app.

---

## 10 · Security Hardening & Virtualisation

*   Memory-safe language everywhere; forbid `unsafe` outside HAL.
*   Sandbox each user-land service with capability filtering & seccomp-like syscall masks.
*   Virtual-machine Monitor (KVM) later hosts legacy guests.

---

## 11 · Portability & Hardware Expansion

*   Abstract HAL targets for RISC-V, aarch64 laptops (M-class), then x86 SVM.
*   Wayland protocol stays identical; only GPU back-ends differ.

---

## 12 · Developer SDK & Community Ecosystem

*   Cargo target triple `--target novel-os.json` plus `novel-sim` QEMU profile.
*   IDEs & Tooling: `vscode-server`, unified debugger with `gdb-stub` over IPC.
*   Contribution Guide: RFC process, lint rules, CI pipeline using `cargo nextest` in emulation.

---

## How This “Starts Small”

1.  Boot + Serial Hello World → validates bootstrap.
2.  Memory + Tasks → run two fibres blinking LEDs/logging.
3.  User Process → load a Unix-style “init” echo server.
4.  Framebuffer Text Console → simple VGA -> later swapped by Wayland compositor.
5.  Package Manager & GUI Shell → after kernel/userland stabilise.
6.  AI Daemon → bolt on once IPC/perf are trustworthy.

Each milestone introduces one new subsystem while tightening tests, docs, and safety guarantees established earlier. The architecture therefore remains lean, auditable, and already “AI-aware” long before heavy models ship.
