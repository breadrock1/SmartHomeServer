# SmartHomeServer
There is simple project based on Rust which created while studying Rust programming language on Otus platform. 

### Target:

 - Learn how to use the networking tools of the standard library.
- Learn to use a separate thread for I / O tasks.
- Learn to develop asynchronous applications.

### The result is:

- A module for working with a smart socket and its simulator. 
- Module for working with a thermometer and its simulator. 
- Asynchronous work modules with a thermometer and a smart socket.

### Description / Step-by-step instructions for completing homework:

1. Smart socket TCP client that allows you to:

 - turn on and off the socket,
 - request information about the current state and power consumption of the outlet.

2. Implement a thermometer that periodically updates temperature data, receiving them from a given socket via UDP. To listen to UDP, it is suggested to run the datagram receive loop in a separate thread.

3. Smart plug and thermometer networking modules must provide an asynchronous interface. System threads should not be created manually. It is recommended to use the tokio library.

4. Also, to test the new functionality, implement an application that simulates the operation of a smart socket and a thermometer controlled by TCP and UDO.

### Criteria for evaluation:

The "Accepted" status is set if:

- There is all the functionality from the description.
- Functional tests from the description are performed.
- The cargo clippy utility does not issue warnings.
- The cargo fmt --check command produces no warnings.

### Competencies:

Infrastructure work
 - use of TCP protocol to implement client-server logic
 - creating system threads and receiving data via UDP
