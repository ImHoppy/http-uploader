mod mimetype;

use futures::TryStreamExt;
use log::*;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;
use warp::{
    filters::multipart::FormData, http::StatusCode, reply, serve, Buf, Filter, Rejection, Reply,
};

const URL_HOST: &str = "http://localhost:8080";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "debug");
    pretty_env_logger::formatted_timed_builder();

    let port = std::env::args()
        .nth(1)
        .map(|val| val.parse::<u16>())
        .unwrap_or(Ok(8080))?;

    let upload_file_route = warp::post()
        .and(warp::path("upload"))
        .and(warp::multipart::form().max_length(200_000_000)) // 200 MB
        .and_then(upload);

    let get_file_route = warp::path!("file" / String).and_then(get_file);

    let static_route = warp::path("static").and(warp::fs::dir("static"));

    let routes = upload_file_route
        .or(get_file_route)
        .or(static_route)
        .recover(handle_rejection);

    info!("Listening on port: {}", port);
    let (_addr, server) = serve(routes).bind_with_graceful_shutdown(([0, 0, 0, 0], port), async {
        tokio::signal::ctrl_c()
            .await
            .expect("http_server: Failed to listen for CRTL+c");
        info!("Shutting down HTTP server");
    });

    server.await;

    Ok(())
}

async fn upload(form: FormData) -> Result<impl Reply, Rejection> {
    let parts: Vec<_> = form
        .and_then(|mut part| async move {
            let id = Uuid::new_v4();
            let extension = part
                .filename()
                .unwrap()
                .split('.')
                .last()
                .unwrap()
                .to_string();

            let mut file = File::create(format!("./uploads/{}.{}", id, extension))
                .await
                .map_err(|e| {
                    trace!("Error creating file: {}", e);
                    warp::reject::reject()
                })
                .unwrap();

            while let Some(data) = part.data().await {
                let data = data.unwrap();
                file.write_all(&data.chunk()).await.unwrap();
            }

            Ok((id.to_string(), extension))
        })
        .try_collect()
        .await
        .map_err(|e| {
            eprintln!("form error: {}", e);
            warp::reject::reject()
        })
        .unwrap();

    let urls: Vec<String> = parts
        .iter()
        .map(|(file, extension)| format!("{URL_HOST}/file/{file}.{extension}"))
        .collect();
    Ok(urls.join("\n"))
}

async fn get_file(file_name: String) -> Result<impl Reply, Rejection> {
    let path = format!("./uploads/{}", file_name);

    let mut file = File::open(path).await.map_err(|e| {
        trace!("Error opening file: {}", e);
        warp::reject::reject()
    })?;

    let mut buf = Vec::new();
    file.read_to_end(&mut buf).await.unwrap();

    let content_type = mimetype::find_mimetype(
        file_name.split('.').last().unwrap(),
        "application/octet-stream",
    );

    Ok(reply::with_header(buf, "content-type", content_type))
}

// Custom rejection handler that maps rejections into responses.
async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    if err.is_not_found() {
        Ok(reply::with_status("NOT_FOUND", StatusCode::NOT_FOUND))
    } else if err.find::<warp::reject::PayloadTooLarge>().is_some() {
        Ok(reply::with_status("BAD_REQUEST", StatusCode::BAD_REQUEST))
    } else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
        Ok(reply::with_status(
            "METHOD_NOT_ALLOWED",
            StatusCode::METHOD_NOT_ALLOWED,
        ))
    } else {
        eprintln!("unhandled rejection: {:?}", err);
        Ok(reply::with_status(
            "INTERNAL_SERVER_ERROR",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}
