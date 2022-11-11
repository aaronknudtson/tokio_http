use tokio::fs::File;
use tokio::io::Result;
use tokio::io::{AsyncRead, BufReader};
use tokio::io::{AsyncReadExt, AsyncWrite, BufWriter};
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
        self.writer
            .write(format!("HTTP/1.1 {} {}\n", code, status).as_bytes())
            .await
    }

    pub async fn write_header(&mut self, key: &str, val: &str) -> Result<usize> {
        self.writer
            .write(format!("\"{}\": {}\n", key, val).as_bytes())
            .await
    }

    pub async fn write_body(&mut self, val: &[u8]) -> Result<usize> {
        self.write_header("content-length", val.len()).await?;
        self.writer.write(b"\n").await?;
        self.writer.write(val).await
    }

    pub fn mime_type(&self, key: &str) -> &str {
        if let Some((_, ext)) = key.rsplit_once(".") {
            match ext {
                "html" => "text/html",
                "css" => "text/css",
                "js" => "text/javascript",
                "jpg" => "image/jpeg",
                "jpeg" => "image/jpeg",
                "png" => "image/png",
                "ico" => "image/x-icon",
                "pdf" => "application/pdf",
                _ => "text/plain",
            }
        } else {
            "text/plain"
        }
    }

    pub fn write_file(&mut self, path: &str) -> Result<()> {
        let file = File::open(path).await?;
        let mut buf = Vec::new(); // with_capacity if we know file size?
        let mut reader = BufReader::new(file);
        reader.read_to_end(&mut buf).await?;

        self.write_header(
            "content-type",
            &format!("{}; charset=UTF-8", self.mime_type(path)),
        ).await?;
        self.write_body(&buf)
    }

    pub fn flush(&mut self) -> Result<()> {
        self.writer.flush()
    }

    pub fn sendfile(&mut self, code: i32, status: &str, path: &str) -> Result<()> {
        self.write_status(code, status).await?;
        self.write_file(path)?;
        self.flush()
    }
}
