mod handlers;
mod mimetype;
mod routes;

use routes::routers;

use clap::{command, Parser};
use log::*;
use std::net::IpAddr;
use warp::serve;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long, default_value = "127.0.0.1", help = "The host to bind to")]
    host: IpAddr,

    #[arg(long, default_value = "8080", help = "The port to bind to")]
    port: u16,

    #[arg(
        long,
        default_value = "./uploads",
        help = "The directory to store uploads"
    )]
    upload_dir: String,

    #[arg(
        long = "url",
        default_value = "http://localhost:8080",
        help = "The URL host"
    )]
    url_host: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::formatted_timed_builder();

    let args = Args::parse();

    info!("Listening on: {} {}", args.host, args.port);

    let (_addr, server) =
        serve(routers(args.clone())).bind_with_graceful_shutdown((args.host, args.port), async {
            tokio::signal::ctrl_c()
                .await
                .expect("http_server: Failed to listen for CRTL+c");
            info!("Shutting down HTTP server");
        });

    server.await;

    Ok(())
}
