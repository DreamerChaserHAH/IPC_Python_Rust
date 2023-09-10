use std::os::unix::net::UnixStream;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()> {
    let mut stream = UnixStream::connect("/tmp/my_socket")?;
    stream.write_all(b"hello world")?;

    let mut response = String::new();
    let mut conn = BufReader::new(stream);
    let _ = conn.read_line(&mut response).expect("unable to read");
    print!("{response}");
    Ok(())
}
