use crate::handlers::handle_rejection;
use crate::handlers::{get_file, upload};
use crate::Args;

use log::*;
use warp::Filter;

fn with_args(
    args: Args,
) -> impl Filter<Extract = (Args,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || args.clone())
}

pub fn routers(args: Args) -> impl Filter<Extract = impl warp::Reply> + Clone {
    let log = warp::log::custom(|info| {
        info!(
            "{} {} {} {}",
            info.remote_addr()
                .map_or("unknown".to_string(), |addr| addr.ip().to_string()),
            info.method(),
            info.path(),
            info.status()
        );
    });

    let optional_param = warp::path::param::<String>()
    .map(Some)
    .or_else(|_| async {
        Ok::<(Option<String>,), std::convert::Infallible>((None,))
    });
    // POST /upload
    let upload = warp::post()
        .and(warp::path("upload"))
        .and(warp::multipart::form().max_length(200_000_000)) // 200 MB
        .and(optional_param)
        .and(with_args(args.clone()))
        .and_then(upload);

    // GET /file/:file_name
    let get_file = warp::path("file")
        .and(warp::path::tail())
        .and(with_args(args.clone()))
        .and_then(get_file);

    // GET /static/:file_name
    let static_files = warp::path("static").and(warp::fs::dir("static"));

    let routers = upload
        .or(get_file)
        .or(static_files)
        .recover(handle_rejection)
        .with(log);

    routers
}
