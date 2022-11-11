use std::io::Result;
use tokio::net::TcpStream;
use std::collections::HashMap;
use crate::node::Node;

#[derive(PartialEq, Eq, Hash)]
pub enum Method {
    GET,
    POST,
    PUT,
    PATCH,
    DELETE
}

pub type HandlerFn = fn(TcpStream) -> Result<()>;
pub struct Router {
    routes: HashMap<Method, Node>
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new()
        }
    }

    pub fn route_client(&self, client: TcpStream) -> Result<()> {
        let mut stream = BufReader::new(socket);
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
            
        } else {
            match (parts[0], parts[1]) {
                ("GET", path) => self.handle(Method::GET, path, client),
                _ => 
            }
        }
    }
    pub fn insert(&mut self, method: Method, path: &str, handler: HandlerFn) {
        let node = self.routes.entry(method).or_insert(Node::new("/"));
        node.insert(path, handler);

    }

    pub fn handle(&self, method: Method, path: &str, client: TcpStream) -> Result<()> {
        if let Some(node) = self.routes.get(&method) {
            if let Some(handler) = node.get(path) {
                return handler(client);
            }
        }
        Ok(())
    }
}
