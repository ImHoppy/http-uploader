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

    #[arg(short, long, default_value = "false", help = "Enable verbose logging")]
    verbose: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let mut builder = pretty_env_logger::formatted_timed_builder();
    builder
        .filter_level(if args.verbose {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .init();

    info!("Listening on: {}:{}", args.host, args.port);

    if !std::path::Path::new(&args.upload_dir).exists() {
        info!("Create upload directory: {}", args.upload_dir);
        match std::fs::create_dir(&args.upload_dir) {
            Ok(_) => (),
            Err(e) => {
                error!("Failed to create upload directory: {}", e);
                std::process::exit(1);
            }
        }
    }

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
