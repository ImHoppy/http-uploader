use crate::handlers::handle_rejection;
use crate::handlers::{get_file, upload};

use warp::Filter;

use crate::Args;

fn with_args(
    args: Args,
) -> impl Filter<Extract = (Args,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || args.clone())
}
pub fn routers(args: Args) -> impl Filter<Extract = impl warp::Reply> + Clone {
    // POST /upload
    let upload = warp::post()
        .and(warp::path("upload"))
        .and(warp::multipart::form().max_length(200_000_000)) // 200 MB
        .and(with_args(args.clone()))
        .and_then(upload);

    // GET /file/:file_name
    let get_file = warp::path("file")
        .and(warp::path::param())
        .and(with_args(args.clone()))
        .and_then(get_file);

    // GET /static/:file_name
    let static_files = warp::path("static").and(warp::fs::dir("static"));

    let routers = upload
        .or(get_file)
        .or(static_files)
        .recover(handle_rejection); // Specify the type of Rejection

    routers
}
