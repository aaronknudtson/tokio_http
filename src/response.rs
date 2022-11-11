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

    pub async fn write_status(&mut self, code: i32, status: &str) -> Result<usize> {
        self.writer.write(format!("HTTP/1.1 {} {}\n", code, status).as_bytes()).await
    }
    
    pub async fn write_header(&mut self, key: &str, val: &str) -> Result<usize> {
        self.writer.write(format!("\"{}\": {}\n", key, val).as_bytes()).await
    }

    pub async fn write_body(&mut self, val: &[u8]) -> Result<usize> {
        self.write_header("content-length", val.len()).await?;
        self.writer.write(b"\n").await?;
        self.writer.write(val).await
    }
}
