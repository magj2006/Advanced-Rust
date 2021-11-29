use color_eyre::Report;
use tracing::info;
use tracing_subscriber::EnvFilter;

pub const URL_1: &str = "https://www.163.com/news/article/GO9F5LTJ000189FH.html?clickfrom=w_yw";
pub const URL_2: &str =
    "https://www.163.com/sports/article/GO9DOTFK00059ADR.html?clickfrom=w_sports";
pub const URL_3: &str = "https://docs.rs/regex/1.5.4/regex/";

mod tj;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Report> {
    setup()?;

    let res = tj::try_join(fetch_thing("first"), fetch_thing("second")).await?;

    info!(?res, "All done!");

    Ok(())
}

async fn fetch_thing(name: &str) -> Result<&str, Report> {
    use std::net::SocketAddr;
    use std::sync::Arc;
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::TcpStream,
    };
    use tokio_rustls::{rustls::ClientConfig, TlsConnector};
    use webpki::DNSNameRef;

    // look out it's port 443 now
    let addr: SocketAddr = ([1, 1, 1, 1], 443).into();
    let socket = TcpStream::connect(addr).await?;

    // establish a TLS session...
    let connector: TlsConnector = {
        let mut config = ClientConfig::new();

        config
            .root_store
            .add_server_trust_anchors(&webpki_roots::TLS_SERVER_ROOTS);

        Arc::new(config).into()
    };

    // we have to use the proper DNS name now
    let dnsname = DNSNameRef::try_from_ascii_str("one.one.one.one")?;
    let mut socket = connector.connect(dnsname, socket).await?;

    // we're writing straight to the socket, there's no buffering
    // so no need to flush
    socket.write_all(b"GET / HTTP/1.1\r\n").await?;
    socket.write_all(b"Host: one.one.one.one\r\n").await?;
    socket.write_all(b"User-Agent: cool-bear\r\n").await?;
    socket.write_all(b"Connection: close\r\n").await?;
    socket.write_all(b"\r\n").await?;

    let mut response = String::with_capacity(256);
    socket.read_to_string(&mut response).await?;

    let status = response.lines().next().unwrap_or_default();

    info!(%status, %name, "Got a response!");

    Ok(name)
}

fn setup() -> Result<(), Report> {
    if std::env::var("RUST_LIB_BACKTRACE").is_err() {
        std::env::set_var("RUST_LIB_BACKTRACE", "1")
    }

    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }

    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    Ok(())
}
