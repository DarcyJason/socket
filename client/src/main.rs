use std::net::TcpStream;
use std::io::{self, Read, Write};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:7878")?;
    println!("Connection to 127.0.0.1:7878 successfully!");

    loop {
        let mut input = String::new();
        println!("Please input your message: (input 'q()' to quit)");
        io::stdout().flush()?;
        io::stdin().read_line(&mut input)?;

        let message = input.trim();
        if message == "q()" {
            break;
        }

        stream.write_all(message.as_bytes())?;
        stream.flush()?;
        println!("Send Message: \"{}\" to the server", message);

        let mut buffer = [0; 1024];
        let n = stream.read(&mut buffer)?;

        if let Ok(response) = String::from_utf8(buffer[..n].to_vec()) {
            println!("Server has received message: {}", response);
        }
    }
    Ok(())
}
