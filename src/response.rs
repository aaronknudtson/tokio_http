use tokio::fs::File;
use tokio::io::Result;
use tokio::io::{AsyncReadExt, BufReader};
use tokio::io::{AsyncWrite, AsyncWriteExt, BufWriter};
use tokio::net::{TcpStream, tcp::WriteHalf};

pub struct Response {
    writer: BufWriter<TcpStream>,
}

pub fn status(code: i32) -> &'static str {
    match code {
        200 => "OK",
        400 => "BAD REQUEST",
        404 => "NOT FOUND",
        _ => "NOT IMPLEMENTED"
    }
}

impl Response {
    pub fn new(client: WriteHalf) -> Self {
        Self {
            writer: BufWriter::new(client),
        }
    }

    pub async fn write_status(&mut self, code: i32) -> Result<usize> {
        self.writer
            .write(format!("HTTP/1.1 {} {}\n", code, status(code)).as_bytes())
            .await
    }

    pub async fn write_header(&mut self, key: &str, val: &str) -> Result<usize> {
        self.writer
            .write(format!("\"{}\": {}\n", key, val).as_bytes())
            .await
    }

    pub async fn write_body(&mut self, val: &[u8]) -> Result<usize> {
        self.write_header("content-length", val.len().into()).await?;
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

    pub async fn write_file(&mut self, path: &str) -> Result<()> {
        let file = File::open(path).await?;
        let mut buf = Vec::new(); // with_capacity if we know file size?
        let mut reader = BufReader::new(file);
        reader.read_to_end(&mut buf).await?;

        self.write_header(
            "content-type",
            &format!("{}; charset=UTF-8", self.mime_type(path)),
        )
        .await?;
        self.write_body(&buf).await?;
        Ok(())
    }

    pub async fn flush(&mut self) -> Result<()> {
        self.writer.flush().await
    }

    pub async fn sendfile(&mut self, code: i32, path: &str) -> Result<()> {
        self.write_status(code).await?;
        self.write_file(path).await?;
        self.flush().await
    }
}
