//client.rs code to test connection
use std::ffi::CString;
use std::io::{Read, Write};
use std::mem::{size_of, zeroed};
use std::net::Ipv4Addr;
use std::os::unix::io::RawFd;
use std::ptr;

use libc::{
    socket, bind, connect, read, write,
    sockaddr_in, in_addr, AF_INET, SOCK_STREAM, sockaddr, c_void, 
};

// port number
const PORT: u16 = 8080;


fn main() {
    unsafe {
        let client_fd: RawFd = socket(AF_INET, SOCK_STREAM, 0);
        if client_fd < 0 {
            panic!("socket() failed");
        }

        let ser_addr = sockaddr_in {
            sin_family: AF_INET as u16,
            sin_port: u16::to_be(PORT),
            sin_addr: in_addr {
                s_addr: u32::from(Ipv4Addr::new(127, 0, 0, 1)).to_be(),
            },
            sin_zero: [0; 8],
        };

        let connection = connect(
            client_fd, 
            &ser_addr as *const _ as *const sockaddr, 
            size_of::<sockaddr_in>() as u32,
        );

        if connection < 0 {
            panic!("connection failed");
        }
        
        loop {
            let message = b"hello from client side";
            let size = write(client_fd, message.as_ptr() as *const c_void, message.len());

            // Buffer to hold the user data
            let mut buffer = [0u8; 1024];
            let bytes_read = read(client_fd, buffer.as_mut_ptr() as *mut c_void, 1024);
            if bytes_read > 0 {
                let message = String::from_utf8_lossy(&buffer[..bytes_read as usize]);
                println!("Received: {}", message);

                // Write back to client
                let response = b"Hello from raw Rust server!\n";
                write(client_fd, response.as_ptr() as *const c_void, response.len());
            }
        }

        // Clean up (close sockets)
        libc::close(client_fd);
    }
}
