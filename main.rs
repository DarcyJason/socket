use std::net::TcpListener;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Server is running on 127.0.0.1:7878 successfully!");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Received connection from: {}", stream.peer_addr()?);
                let mut buffer = [0; 1024];

                while let Ok(n) = stream.read(&mut buffer) {
                    if n == 0 {
                        println!("client disconnected!");
                        break;
                    }

                    if let Ok(message) = String::from_utf8(buffer[..n].to_vec()) {
                        println!("Received message: {} from {}", message, stream.peer_addr()?);
                        stream.write_all(message.as_bytes())?;
                        stream.flush()?;
                    }
                }
            }
            Err(e) => {
                eprintln!("Accept connection failed: {}", e);
            }
        }
    }
    Ok(())
}
