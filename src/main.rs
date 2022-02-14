use std::error::Error;
use std::io::prelude::*;
use std::net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream};
use std::{thread, time};

pub fn main() {
    if let Err(e) = run(4050) {
        eprint!("{}", e);
        std::process::exit(1);
    };
}

pub fn run(port: u16) -> Result<(), Box<dyn Error>> {
    let socket = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port);
    let listener = TcpListener::bind(socket)?;

    println!("Listening on port {}", port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Connection established!");
                handle_client(stream)?;
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buffer = [0; 512];
    let byte_count = stream.read(&mut buffer)?;

    println!("Received {} bytes", byte_count);

    let response = String::from_utf8_lossy(&buffer[..]);
    println!("Received: {}", response);

    println!("Sleeping for 5 seconds, Close netcat on the other end");
    thread::sleep(time::Duration::from_secs(5));

    println!("Will try to send response. THIS SHOULD FAIL!");
    stream.write_all(response.as_bytes())?;
    stream.flush()?;
    println!("Expected errrors were not returned!");

    Ok(())
}
