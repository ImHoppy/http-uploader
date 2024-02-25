use std::fs;
use std::io;
use std::path::Path;
use futures::TryStreamExt;
use warp::{
    reply, Reply, Filter, reject,
    Rejection,
    http::StatusCode,
    filters::multipart::{FormData, Part}
};

#[tokio::main]
async fn main() {
    let upload_route = warp::post()
        .and(warp::path("upload"))
        .and(warp::multipart::form().max_length(5_000_000))
        .and_then(upload);

    let routes = upload_route.recover(handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn upload(form: FormData) -> Result<impl Reply, Rejection> {
    let parts: Vec<Part> = form.try_collect().await.map_err(|e| {
        eprintln!("form error: {}", e);
        warp::reject::reject()
    })?;
    
    for part in parts {
        println!("name = {}", part.name());
    }

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
        Ok(reply::with_status("INTERNAL_SERVER_ERROR", StatusCode::INTERNAL_SERVER_ERROR))
    }
}
