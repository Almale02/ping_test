use std::{
    net::{SocketAddr, TcpStream},
    str::FromStr as _,
    time::{Duration, Instant},
};

pub fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if let Some(addr) = args.get(1) {
        let addr = SocketAddr::from_str(addr).unwrap();
        let mut stream = TcpStream::connect(addr).unwrap();
        println!("connected to server: {}", addr);

        stream
            .set_read_timeout(Some(Duration::from_secs(5)))
            .unwrap();
        stream
            .set_write_timeout(Some(Duration::from_secs(5)))
            .unwrap();

        let mut ping_count = 0;
        let mut successes = 0;
        let start_time = Instant::now();
    }
}
