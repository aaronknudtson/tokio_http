use std::collections::HashMap;
use std::fmt::Display;
use std::io::Cursor;

use anyhow::Result;
use bytes::{Buf, Bytes, BytesMut};
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, AsyncBufReadExt};
use tokio::net::{TcpListener, TcpStream};

enum Method {
    Get,
    Put,
    Post,
    Patch,
    Delete,
}

type HeaderMap = HashMap<String, String>;
enum HttpFrame {
    RequestHead {
        method: Method,
        uri: String,
        version: String,
        headers: HeaderMap,
    },
    ResponseHead {
        status: u16,
        version: String,
        headers: HeaderMap,
    },
    BodyChunk {
        chunk: Bytes,
    },
}

#[derive(Debug)]
enum Error {
    Incomplete,
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Incomplete")
    }
}

impl std::error::Error for Error {}

fn get_u8(src: &mut Cursor<&[u8]>) -> Result<u8, Error> {
    if !src.has_remaining() {
        return Err(Error::Incomplete);
    }

    Ok(src.get_u8())
}

fn get_line<'a>(src: &mut Cursor<&'a [u8]>) -> Result<&'a [u8], Error> {
    // Scan the bytes directly
    let start = src.position() as usize;
    // Scan to the second to last byte
    let end = src.get_ref().len() - 1;

    for i in start..end {
        if src.get_ref()[i] == b'\r' && src.get_ref()[i + 1] == b'\n' {
            // We found a line, update the position to be *after* the \n
            src.set_position((i + 2) as u64);

            // Return the line
            return Ok(&src.get_ref()[start..i]);
        }
    }

    Err(Error::Incomplete)
}

// impl HttpFrame {
//     pub fn check(src: &mut Cursor<&[u8]>) -> Result<()> {
//         match get_u8(src)? {
//             b'+' => {
//                 get_line(src)?;
//                 Ok(())
//             }
//             b'-' => {
//                 get_line(src)?;
//                 Ok(())
//             }
//             b':' => {
//                 let _ = get_decimal(src)?;
//                 Ok(())
//             }
//             b'$' => {
//                 if b'-' == peek_u8(src)? {
//                     // Skip '-1\r\n'
//                     skip(src, 4)
//                 } else {
//                     // Read the bulk string
//                     let len: usize = get_decimal(src)?.try_into()?;
//
//                     // skip that number of bytes + 2 (\r\n).
//                     skip(src, len + 2)
//                 }
//             }
//             b'*' => {
//                 let len = get_decimal(src)?;
//
//                 for _ in 0..len {
//                     Frame::check(src)?;
//                 }
//
//                 Ok(())
//             }
//             actual => Err(format!("protocol error; invalid frame type byte `{}`", actual).into()),
//         }
//     }
// }

pub struct Connection {
    stream: TcpStream,
    buffer: BytesMut,
}

// impl Connection {
//     pub fn new(stream: TcpStream) -> Connection {
//         Connection {
//             stream,
//             // Allocate the buffer with 4kb of capacity.
//             buffer: BytesMut::with_capacity(4096),
//         }
//     }
//
//     pub async fn read_frame(&mut self) -> Result<Option<HttpFrame>> {
//         loop {
//             if let Some(frame) = self.parse_frame()? {
//                 return Ok(Some(frame));
//             }
//
//             if 0 == self.stream.read_buf(&mut self.buffer).await? {
//                 if self.buffer.is_empty() {
//                     return Ok(None);
//                 } else {
//                     return Err(anyhow::Error::msg("connection reset by peer"));
//                 }
//             }
//         }
//     }
//
//     fn parse_frame(&mut self) -> Result<Option<HttpFrame>> {
//         let mut buf = Cursor::new(&self.buffer[..]);
//         match HttpFrame::check(&mut buf) {
//             Ok(_) => {
//                 // Get the byte length of the frame
//                 let len = buf.position() as usize;
//
//                 // Reset the internal cursor for the
//                 // call to `parse`.
//                 buf.set_position(0);
//
//                 // Parse the frame
//                 let frame = HttpFrame::parse(&mut buf)?;
//
//                 // Discard the frame from the buffer
//                 self.buffer.advance(len);
//
//                 // Return the frame to the caller.
//                 Ok(Some(frame))
//             }
//             // Not enough data has been buffered
//             Err(Incomplete) => Ok(None),
//             // An error was encountered
//             Err(e) => Err(e.into()),
//         }
//     }
// }

// struct Request {
//     stream: TcpStream,
//     req_type:
// }
//
// impl Request {
//     fn parse_type() -> HttpRequest {
//         todo!()
//     }
// }

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:42069";
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on {}", addr);

    loop {
        let (socket, _) = listener.accept().await?;
        let mut stream = BufReader::new(socket);
        tokio::spawn(async move {
            match stream.lines().next_line().await {
                Ok(Some(s)) => {println!("{}", s)},
                Ok(None) => {},
                Err(_) => todo!()
            }
            // let mut buf = Vec::with_capacity(1024);
            // match stream.read(&mut buf).await {
            //     Ok(0) => {
            //         println!("None {:?}", buf);
            //     }
            //     Ok(_) => {
            //         println!("{:?}", buf);
            //     }
            //     Err(e) => {
            //         eprintln!("Encountered error: {}", e);
            //     }
            // }
        });
    }
}
