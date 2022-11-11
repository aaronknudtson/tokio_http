use tokio::io::{BufWriter, AsyncWrite};
use tokio::io::{BufReader, AsyncRead};
use tokio::io::Result;
use tokio::net::TcpStream;

pub struct Response {
    writer: BufWriter<TcpStream>,
}

impl Response {
    pub fn new(client: TcpStream) -> Self {
        Self {
            writer: BufWriter::new(client),
        }
    }

    pub fn write_status(&mut self, code: i32, status: &str)
}
