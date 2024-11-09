use tokio::{io::copy_bidirectional, net::TcpStream};

pub async fn proxy(mut client: TcpStream, remote_addr: &str) {
    match TcpStream::connect(remote_addr).await {
        Ok(mut server) => {
            if copy_bidirectional(&mut client, &mut server).await.is_err() {
                eprintln!("Failed to copy data between client and server");
            }
        }
        Err(e) => {
            eprintln!("Failed to connect to remote server: {}", e);
        }
    }
}
