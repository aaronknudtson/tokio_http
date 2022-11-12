use std::future::Future;
use std::pin::Pin;
use std::io::Result;
use tokio::net::TcpStream;
use tokio::io::{BufReader, AsyncRead, AsyncBufReadExt};
use std::collections::HashMap;
use crate::node::Node;
use crate::response::Response;

#[derive(PartialEq, Eq, Hash)]
pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE
}

pub type HandlerRes = Pin<Box<dyn Future<Output=Result<()>>>>;
pub type HandlerFn = Box<fn(&mut TcpStream) ->  HandlerRes>;
pub struct Router {
    routes: HashMap<Method, Node>
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new()
        }
    }

    pub async fn route_client(&self, client: &mut TcpStream) -> Result<()> {
        let (reader, writer) = client.split();
        let mut stream = BufReader::new(reader);
        let buf = stream.fill_buf().await?;

        // read a single line if one exists
        let mut line = String::new();
        let mut line_reader = BufReader::new(buf);

        // consume bytes
        let len = line_reader.read_line(&mut line).await?;
        if len == 0 {
            return Ok(());
        }

        let parts: Vec<&str> = line.split(" ").collect();
        if parts.len() < 2 {
            let mut res = Response::new(writer);
            res.sendfile(400, "static/_400.html").await
        } else {
            match (parts[0], parts[1]) {
                ("GET", path) => self.handle(Method::GET, path, client).await,
                _ => {
                    let mut res = Response::new(writer);
                    res.sendfile(404, "static/_404.html").await
                }
            }
        }
    }
    pub fn insert(&mut self, method: Method, path: &str, handler: HandlerFn) {
        let node = self.routes.entry(method).or_insert(Node::new("/"));
        node.insert(path, handler);

    }

    pub async fn handle(&self, method: Method, path: &str, client: &mut TcpStream) -> Result<()> {
        if let Some(node) = self.routes.get(&method) {
            if let Some(handler) = node.get(path) {
                return handler(client).await;
            }
        }
        Ok(())
    }
}
