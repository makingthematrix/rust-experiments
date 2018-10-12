use std::net::TcpListener;

use std::env;
use std::net::SocketAddr;

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

mod core;
mod streams;
pub use self::core::handle;

pub fn main() {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:4141".to_string());
    let addr = addr.parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    let queue = Arc::new(Mutex::new(VecDeque::new()));

    listener
        .incoming()
        .filter_map(|res| match res {
            Err(err) => {
                println!("error: {:?}", err);
                None
            }
            Ok(r) => Some(r),
        })
        .for_each(|mut stream| {
            println!("accepted stream; addr={:?}", stream.peer_addr().unwrap());
            let q = queue.clone();
            let mut q_locked = q.lock().unwrap();
            if let Err(err) = core::handle(&mut stream, &mut q_locked) {
                println!("error: {:?}", err);
            }
        });
}
