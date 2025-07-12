mod tcp;

use tcp::{Server, Ipv, Conn};
use chrono::Utc;

fn main() {
        let now = Utc::now().to_rfc3339();
        println!("INFO [timestamp={}]: Starting Server", now);

        let server = Server {
            ipv: Ipv::V4,
            conn: Conn::Tcp,
            port: 8080,
            addr: "127.0.0.1".to_string(),
            listen_queue: 5,
        };

        match server.init() {
            Ok(mut socket_info) => {
                let mut client_fd;
                loop {
                    client_fd = Server::accept_connection(&mut socket_info);
                    Server::handle_connection(client_fd);
                }
                // Clean up (close sockets)
                unsafe {
                    //libc::close(client_fd);
                    libc::close(socket_info.fd);
                }
            }
            Err(err) => eprintln!("Server failed: {}", err),
        }
}

