#![feature(duration_millis_float)]

use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpStream},
    process::exit,
    str::FromStr as _,
    thread,
    time::{Duration, Instant},
};

pub fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if let Some(addr) = args.get(1) {
        let addr = SocketAddr::from_str(addr).unwrap();
        let mut stream = TcpStream::connect_timeout(&addr, Duration::from_secs(5)).unwrap();
        println!("connected to server: {}", addr);

        let mut pinging_time = Duration::ZERO;
        let mut ping_count = 0;
        let mut resp_buffer = [0_u8; 1024];

        loop {
            let ping_start = Instant::now();
            match stream.write_all(b"ping req") {
                Ok(..) => match stream.read(&mut resp_buffer) {
                    Ok(0) => {
                        eprintln!("server closed");
                        exit(-1);
                    }
                    Ok(buff_len) => {
                        let data = &resp_buffer[0..buff_len];
                        if data == b"ping resp" {
                            ping_count += 1;
                            let elapsed = ping_start.elapsed();
                            pinging_time += elapsed;
                            println!(
                                "Ping {:.2}ms, avarage: {}",
                                elapsed.as_millis_f32(),
                                pinging_time.as_millis_f32() / ping_count as f32
                            );
                        } else {
                            eprintln!(
                                "invalid response from server: {}",
                                String::from_utf8_lossy(data)
                            );
                        }
                    }
                    Err(err) => {
                        if err.kind() == std::io::ErrorKind::WouldBlock {
                            eprintln!("response time out");
                            exit(-1);
                        } else {
                            eprintln!("response error: {}", err);
                            exit(-1);
                        }
                    }
                },
                Err(e) => {
                    eprintln!("error: {}, failed to send ping, reconnecting...", e);
                    thread::sleep(Duration::from_millis(2000));
                    match TcpStream::connect_timeout(&addr, Duration::from_secs(5)) {
                        Ok(new_stream) => {
                            stream = new_stream;
                            println!("reconnectd to server");
                        }
                        Err(e) => {
                            eprintln!("error: {}, failed to reconnect...", e);
                        }
                    }
                }
            }
            thread::sleep(Duration::from_millis(500));
        }
        //println!("avarage ping: {}ms", sta);
    }
}
/*





















*/
