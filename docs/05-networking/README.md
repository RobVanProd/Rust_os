# 5 · Networking

This stage introduces networking capabilities to the operating system, enabling communication with other systems.

1.  **Network Daemon (Net Daemon)**
    *   Networking will primarily be handled by a daemon running in user land for security and modularity.
    *   An Ethernet driver (interfacing with physical hardware or a virtual NIC) will send/receive packets.
    *   Packets will be passed to/from the driver via a virtual queue or similar mechanism.
    *   A TCP/IP stack, likely based on a library like `smoltcp`, will be integrated into this daemon.

2.  **IPC Bridge for Network Access**
    *   An IPC bridge will expose asynchronous socket-like interfaces to applications.
    *   Applications will communicate with the Net Daemon via IPC to perform network operations (e.g., connect, send, receive).

Upon completion, applications running on the OS will be able to access network services, and the OS itself can participate in network communication.
