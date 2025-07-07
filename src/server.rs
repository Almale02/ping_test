use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpListener},
    process::exit,
    str::FromStr,
    time::Duration,
};

pub fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if let Some(addr) = args.get(1) {
        let addr = SocketAddr::from_str(addr).unwrap();
        let listener = TcpListener::bind(addr).unwrap();
        println!("listening on: {}", addr);

        loop {
            println!("waiting for client");
            let (mut stream, client_addr) = listener.accept().unwrap();
            println!("new connection from {}", client_addr);
            stream
                .set_read_timeout(Some(Duration::from_secs(5)))
                .unwrap();
            stream
                .set_write_timeout(Some(Duration::from_secs(5)))
                .unwrap();

            let mut req_buffer = [0_u8; 2048];

            loop {
                match stream.read(&mut req_buffer) {
                    Ok(0) => {
                        println!("client {} disconnected", client_addr);
                        break;
                    }
                    Ok(req_len) => {
                        let data = &req_buffer[0..req_len];
                        if data == b"ping req" {
                            match stream.write_all(b"ping response") {
                                Ok(_) => (),
                                Err(_) => {
                                    eprintln!("failed to send response");
                                    exit(-1);
                                }
                            }
                        } else {
                            eprintln!("invalid request");
                            exit(-1);
                        }
                    }
                    Err(err) => {
                        if err.kind() == std::io::ErrorKind::WouldBlock {
                            eprintln!("request time out");
                            exit(-1);
                        } else {
                            eprintln!("other request error");
                            exit(-1);
                        }
                    }
                }
            }
        }
    }
}
