use crate::mimetype;
use crate::Args;

use std::fs::create_dir;
use std::path::Path;

use filetime::set_file_atime;
use futures::TryStreamExt;
use log::*;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use url::Url;
use uuid::Uuid;
use warp::filters::path::Tail;
use warp::reject;
use warp::{filters::multipart::FormData, http::StatusCode, reply, Buf, Rejection, Reply};

#[derive(Debug)]
struct BadRequest;

impl reject::Reject for BadRequest {}

pub async fn upload(form: FormData, folder_param: Option<String>, args: Args) -> Result<impl Reply, Rejection> {
    if let Some(folder_param) = &folder_param {
        if !folder_param.chars().all(|c: char| c.is_ascii_alphabetic()) || folder_param.len() > 16 {
            return Err(warp::reject::custom(BadRequest));
        }
    }
    let folder_param = folder_param.map_or("".to_string(), |param| param.to_ascii_lowercase());

    let parts: Vec<_> = form
        .and_then(move |mut part|  {
            let id = Uuid::new_v4();
            let extension = part
                .filename()
                .unwrap()
                .split('.')
                .last()
                .unwrap()
                .to_string();

            let file_name = format!("{}/{}.{}", folder_param, id, extension);

            let upload_dir = args.upload_dir.clone();

            if folder_param.len() > 0 {
                create_dir(format!("{}/{}", upload_dir, folder_param)).unwrap_or_default();
            }
            async move {
                let mut file = File::create(format!("{}/{}", upload_dir, file_name))
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

                Ok(file_name)
            }
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
        .map(|file_name| {
            let pathname = format!("/file/{}", file_name);
            let url = Url::parse(&args.url_host)?.join(&pathname)?;

            Ok(url.to_string())
        })
        .filter_map(|url: Result<String, url::ParseError>| match url {
            Ok(url) => Some(url),
            Err(e) => {
                error!("url: {}", e);
                None
            }
        })
        .collect();
    Ok(urls.join("\n"))
}

pub async fn get_file(file_name: Tail, args: Args) -> Result<impl Reply, Rejection> {
    let path_str = format!("{}/{}", args.upload_dir, file_name.as_str());

    let path = Path::new(&path_str);
    println!("path: {:?}", path);
    let mut file = File::open(path).await.map_err(|e| {
        trace!("Error opening file: {}", e);
        warp::reject::not_found()
    })?;

    set_file_atime(path, filetime::FileTime::now()).unwrap();

    let mut buf = Vec::new();
    file.read_to_end(&mut buf).await.unwrap();

    let content_type = mimetype::find_mimetype(
        file_name.as_str().split('.').last().unwrap(),
        "application/octet-stream",
    );

    Ok(reply::with_header(buf, "content-type", content_type))
}

// Custom rejection handler that maps rejections into responses.
pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
    if err.is_not_found() {
        Ok(reply::with_status("NOT_FOUND", StatusCode::NOT_FOUND))
    } else if err.find::<warp::reject::PayloadTooLarge>().is_some() || err.find::<BadRequest>().is_some() {
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
