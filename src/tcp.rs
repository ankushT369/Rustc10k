//use std::ffi::CString;
use std::io::{Read, Write};
use std::mem::{size_of, zeroed};
use std::net::Ipv4Addr;
use std::os::unix::io::RawFd;
use chrono::Utc;
//use std::ptr;

use libc::{
    socket, bind, listen, accept, read, write,
    sockaddr_in, in_addr, AF_INET, AF_INET6, 
    SOCK_STREAM, sockaddr, c_void, 
};

#[derive(Copy, Clone)]
#[repr(i32)]
pub enum Ipv {
    V4 = AF_INET,
    V6 = AF_INET6,
}

#[derive(Copy, Clone)]
#[repr(i32)]
pub enum Conn {
    Tcp = SOCK_STREAM,
}

pub struct Server {
    pub ipv: Ipv, 
    pub conn: Conn,
    pub port: u16,
    pub addr: String,
    pub listen_queue: i32,
}

pub struct ServerSocket {
    pub fd: RawFd,
    pub client_addr: sockaddr_in,
    pub addr_len: u32,
    pub port: u16,
}

impl Server {
    pub fn init(&self) -> Result<ServerSocket, String> {
        unsafe {
            let now = Utc::now().to_rfc3339();

            let server_fd: RawFd = socket(self.ipv as i32, self.conn as i32, 0);
            if server_fd < 0 {
                return Err("socket() failed".into());
            }

            let ip: Ipv4Addr = self.addr.parse()
                .map_err(|_| "Invalid IP address".to_string())?;

            // Setup address
            let addr = sockaddr_in {
                sin_family: self.ipv as u16,
                sin_port: self.port.to_be(), // host to network byte order
                sin_addr: in_addr {
                    s_addr: u32::from(ip).to_be(),
                },
                sin_zero: [0; 8],
            };

            // Bind socket to IP/Port
            let bind_result = bind(
                server_fd,
                &addr as *const _ as *const sockaddr,
                size_of::<sockaddr_in>() as u32,
            );
            if bind_result < 0 {
                return Err("bind() failed".into());
            }

            // Listen on socket
            if listen(server_fd, self.listen_queue) < 0 {
                panic!("listen() failed");
            }

            println!("INFO [timestamp={}]: Server is listening on {}:{}",
                now, self.addr, self.port);

            Ok(ServerSocket {
                fd: server_fd,
                client_addr: std::mem::zeroed(), // zeroed but initialized
                addr_len: size_of::<sockaddr_in>() as u32,
                port: self.port,
            })
        }
    }

    pub fn accept_connection(sock: &mut ServerSocket) -> RawFd {
        unsafe {
            let now = Utc::now().to_rfc3339();
            let client_fd = accept(
                sock.fd,
                &mut sock.client_addr as *mut _ as *mut sockaddr,
                &mut sock.addr_len as *mut u32,
            );
            if client_fd < 0 {
                println!("accept() failed");
                return 0;
            }

            println!("INFO [timestamp={}]: Client of fd: {} connected at port: {}!", now, client_fd, sock.port);
            return client_fd;
        }
    }

    pub fn handle_connection(client_fd: RawFd) {
        let mut buffer = [0u8; 1024];
        unsafe {
            loop {
                let bytes_read = read(
                    client_fd,
                    buffer.as_mut_ptr() as *mut c_void,
                    buffer.len(),
                );

                if bytes_read < 0 {
                    println!("ERROR: Failed to read from client fd: {}", client_fd);
                    libc::close(client_fd);
                    break;
                } else if bytes_read == 0 {
                    // Client closed connection
                    let now = Utc::now().to_rfc3339();
                    println!(
                        "INFO [timestamp={}]: Client fd: {} disconnected.",
                        now, client_fd
                    );
                    break;
                } else {
                    // Print the message
                    let message = String::from_utf8_lossy(&buffer[..bytes_read as usize]);
                    println!(
                        "INFO: Received from client fd {}: {}",
                        client_fd, message
                    );

                    // Echo back to client
                    let response = b"Echo: ";
                    write(client_fd, response.as_ptr() as *const c_void, response.len());
                    write(
                        client_fd,
                        message.as_bytes().as_ptr() as *const c_void,
                        message.len(),
                    );
                }
            }
            libc::close(client_fd);
        }
    }

}
