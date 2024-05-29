use futures::TryStreamExt;
use log::*;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;
use warp::{
    filters::multipart::{FormData, Part},
    http::StatusCode,
    reply, serve, Buf, Filter, Rejection, Reply,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::formatted_timed_builder();

    let port = std::env::args()
        .nth(1)
        .map(|val| val.parse::<u16>())
        .unwrap_or(Ok(8080))?;

    let upload_file_route = warp::post()
        .and(warp::path("upload"))
        .and(warp::multipart::form().max_length(5_000_000))
        .and_then(upload);

    let get_file_route = warp::path!("file" / String).and_then(get_file);

    let routes = upload_file_route
        .or(get_file_route)
        .recover(handle_rejection);

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
    let id = Uuid::new_v4();

    let mut parts: Vec<Part> = form
        .try_collect()
        .await
        .map_err(|e| {
            eprintln!("form error: {}", e);
            warp::reject::reject()
        })
        .unwrap();

    let part = parts.get_mut(0).unwrap();

    let extension = part.filename().unwrap().split('.').last().unwrap();

    let mut file = File::create(format!("./uploads/{}.{}", id, extension))
        .await
        .map_err(|e| {
            eprintln!("file error: {}", e);
            warp::reject::reject()
        })?;

    if let Some(data) = part.data().await {
        let data = data.unwrap();
        file.write_all(&data.chunk()).await.unwrap();
    }

    info!("Upload: {}", id);

    Ok(id.to_string())
}

async fn get_file(file_name: String) -> Result<impl Reply, Rejection> {
    Ok("success")
}

/*
fn save_file(
    name: &str,
    filename: &str,
    content_type: String,
    data: &[u8],
) -> Result<(), io::Error> {
    let upload_dir = "./uploads";
    let filepath = format!("{}/{}", upload_dir, filename);
    // Create the upload directory if it doesn't exist
    fs::create_dir_all(Path::new(upload_dir))?;
    // Write the data to the file
    fs::write(filepath, data)?;
    Ok(())
}
*/

// Custom rejection handler that maps rejections into responses.
async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    if err.is_not_found() {
        Ok(reply::with_status("NOT_FOUND", StatusCode::NOT_FOUND))
    } else if err.find::<warp::reject::PayloadTooLarge>().is_some() {
        Ok(reply::with_status("BAD_REQUEST", StatusCode::BAD_REQUEST))
    } else {
        eprintln!("unhandled rejection: {:?}", err);
        Ok(reply::with_status(
            "INTERNAL_SERVER_ERROR",
            StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}
