#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:6379").await?;
    loop {
        let (stream, addr) = listener.accept().await?;
        eprintln!("accepted: {addr:?}");
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream).await {
                eprintln!("Err({e:?}");
            }
        });
    }
}

const PONG: &[u8] = "+PONG\r\n".as_bytes();

async fn handle_connection(mut stream: tokio::net::TcpStream) -> tokio::io::Result<()> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let (mut rx, mut tx) = stream.split();
    let mut buf = [0; 4096];
    loop {
        match rx.read(&mut buf).await {
            Ok(0) => {
                return Ok(());
            }
            Ok(_) => {
                tx.write_all(PONG).await?;
                tx.flush().await?;
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
}
