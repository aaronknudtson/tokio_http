use crate::router::{Router, HandlerFn, HandlerRes};
use crate::response::Response;
use tokio::net::TcpStream;
use tokio::io::Result;

pub fn configure(router: &mut Router) {
    router.insert(crate::router::Method::GET, "/", index);
}

async fn index(client: &'static mut TcpStream) -> HandlerRes {
    Box::pin(async move {
        let (reader, writer) = client.split();
        let mut res = Response::new(writer);
        res.sendfile(200, "static/index.html").await
    })
}
