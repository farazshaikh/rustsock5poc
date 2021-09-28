use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
    runtime::Runtime,
};
use tokio_socks::{tcp::Socks5Stream, Error};
const TCP_PROXY_ADDR: &str = "127.0.0.1:9011";
const TARGET_SERVER: &str = "192.168.1.25";
const TARGET_PORT: u16 = 3333;

async fn connect_using_proxy() -> Result<(), Error> {
    println!("connect started");
    let s = TcpStream::connect(TCP_PROXY_ADDR).await?;
    println!("connect complete");
    s.set_nodelay(true).unwrap();
    let mut conn = Socks5Stream::connect_with_socket(s, (TARGET_SERVER, TARGET_PORT)).await?;
    println!("target connect complete");
    let send_data = String::from("hello");
    let mut read_data = [0 as u8; 5];
    conn.write_all(send_data.as_bytes()).await.unwrap();
    conn.read_exact(&mut read_data[..]).await.unwrap();
    println!(
        "Read back {:?}",
        std::str::from_utf8(&read_data[..]).unwrap()
    );
    Ok(())
}

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(connect_using_proxy()).unwrap();
}
