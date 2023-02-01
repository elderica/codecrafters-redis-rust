use std::io::{self, Read, Write};
use std::{net, result};

fn split_resp(resp: &[u8]) -> Vec<String> {
    resp.split(|b| *b == 0x0d || *b == 0x0a)
        .map(|s| String::from_utf8(s.to_vec()).expect("should not be invalid bytes for utf8"))
        .collect()
}

const PONG: &[u8] = "+PONG\r\n".as_bytes();

fn main() -> result::Result<(), io::Error> {
    let listener = net::TcpListener::bind("127.0.0.1:6379")?;

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                eprintln!("accepted new connection");

                let mut buf = [0; 64];
                _stream.read_exact(&mut buf)?;

                let parsed = split_resp(&buf);
                eprintln!("{:?}", parsed);

                _stream.write_all(PONG)?;
            }
            Err(e) => {
                eprintln!("error: {}", e);
            }
        }
    }

    Ok(())
}
