use tokio::{net::{TcpListener, TcpStream}, io::{self, AsyncWriteExt, AsyncReadExt}};

const BUFFER_SIZE: usize = 128;

#[tokio::main]
async fn main() -> io::Result<()> {

    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            echo_stream(socket).await
            .expect("There was a problem handling a socket");
        });
    }
}

async fn echo_stream(mut socket: TcpStream) -> io::Result<()> {
    
    println!("Handling a request stream: {:?}", socket);

    let mut buf = [0; BUFFER_SIZE];
    
    // handle error case here
    loop {
        let num_read_bytes = socket.read(&mut buf).await?;
        if num_read_bytes == 0 {
            println!("Done handling request stream: {:?}", socket);
            return Ok(());
        }

        // handle error case here
        socket.write_all(&buf[0..num_read_bytes]).await?
    }
}