# 0 · Vision & Core Principles

| Goal                               | Design Choice                                                                                                | Rationale                                                                                                     |
| ---------------------------------- | ------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------- |
| Memory-safe, micro-/hybrid-kernel  | All internal code in safe Rust with `#![no_std]`; kernel < 50 k SLOC; drivers ideally in user space          | Minimises whole-system exploit surface while retaining performance (see Theseus OS)                             |
| Capability-based security          | Kernel hands out cap tokens (send-once, shareable, revocable) instead of broad syscalls                        | Lets AI sub-services run safely with elevated privileges only where needed                                    |
| Message-passing first              | Zero-copy channels between tasks; asynchronous (async/await-friendly)                                        | Plays well with Rust futures and later distributed AI daemons                                                 |
| AI-ready substrate                 | Standard “AI bus” protocol baked into IPC layer; dynamic model registry                                      | Allows drop-in inference back-ends (Candle, Burn, ONNX-RT) without kernel patches                             |
| Wayland-style graphics stack       | Compositor in user space, protocol built with Smithay blocks                                                 | Modern, departs from X11 complexity, written in Rust                                                          |
| Modern GUI tool-kit                | Widget library based on iced/libcosmic plus wgpu renderer                                                    | Proven in Pop!_OS COSMIC; easily themed; GPU-accelerated                                                      |

This section outlines the foundational goals and design choices that will guide the development of the Rust-Native GUI Operating System. Each principle is chosen to contribute towards a memory-safe, secure, performant, and AI-ready system.
