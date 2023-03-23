
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{io::Error};

const BUFFER_SIZE: usize = 128;

fn main() -> Result<(), Error> {
    let listener = TcpListener::bind("127.0.0.1:3000")?;

    for stream in listener.incoming() {
        
        // handle error case here
        echo_stream(stream?)?;
    }

    Ok(())
}

fn echo_stream(mut stream: TcpStream) -> Result<(), Error> {
    
    println!("Handling a request stream: {:?}", stream);

    let mut buf = [0; BUFFER_SIZE];
    
    // handle error case here
    loop {
        let num_read_bytes = stream.read(&mut buf)?;
        if num_read_bytes == 0 {
            println!("Done handling request stream: {:?}", stream);
            return Ok(());
        }

        // handle error case here
        stream.write_all(&buf[0..num_read_bytes])?
    }
}