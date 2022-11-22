use std::env;
use std::net::{SocketAddr, SocketAddrV4};
use std::str::FromStr;

mod argparse;

#[tokio::main]
async fn main() {
    let args = argparse::parse();

    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info")
    }
    pretty_env_logger::init();

    let address: SocketAddr = match args.bind {
        Some(addr) => SocketAddr::V4(SocketAddrV4::from_str(&addr).unwrap()),
        _ => ([127, 0, 0, 1], args.port).into(),
    };

    warp::serve(warp::fs::dir(args.root_dir)).run(address).await;
}
