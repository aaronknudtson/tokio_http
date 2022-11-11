use crate::router::Router;
use tokio::net::TcpStream;
use tokio::io::Result;

pub fn configure(router: &mut Router) {
    router.insert("GET", "/", index);
}

fn index(client: TcpStream) -> Result<()> {
    todo!();
}
