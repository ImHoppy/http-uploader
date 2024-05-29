pub fn find_mimetype(extension: &str, default_content_type: &str) -> String {
    match extension {
        "css" => "text/css".to_string(),
        "csv" => "text/csv".to_string(),
        "htm" | "html" => "text/html".to_string(),
        "js" | "mjs" => "text/javascript".to_string(),
        "txt" => "text/plain".to_string(),
        "vtt" => "text/vtt".to_string(),

        "apng" => "image/apng".to_string(),
        "avif" => "image/avif".to_string(),
        "bmp" => "image/bmp".to_string(),
        "gif" => "image/gif".to_string(),
        "png" => "image/png".to_string(),
        "svg" => "image/svg+xml".to_string(),
        "webp" => "image/webp".to_string(),
        "ico" => "image/x-icon".to_string(),
        "tif" | "tiff" => "image/tiff".to_string(),
        "jpg" | "jpeg" => "image/jpeg".to_string(),

        "mp4" => "video/mp4".to_string(),
        "mpeg" => "video/mpeg".to_string(),
        "webm" => "video/webm".to_string(),

        "mp3" => "audio/mp3".to_string(),
        "mpga" => "audio/mpeg".to_string(),
        "weba" => "audio/webm".to_string(),
        "wav" => "audio/wave".to_string(),

        "otf" => "font/otf".to_string(),
        "ttf" => "font/ttf".to_string(),
        "woff" => "font/woff".to_string(),
        "woff2" => "font/woff2".to_string(),

        "7z" => "application/x-7z-compressed".to_string(),
        "atom" => "application/atom+xml".to_string(),
        "pdf" => "application/pdf".to_string(),
        "json" => "application/json".to_string(),
        "rss" => "application/rss+xml".to_string(),
        "tar" => "application/x-tar".to_string(),
        "xht" | "xhtml" => "application/xhtml+xml".to_string(),
        "xslt" => "application/xslt+xml".to_string(),
        "xml" => "application/xml".to_string(),
        "gz" => "application/gzip".to_string(),
        "zip" => "application/zip".to_string(),
        "wasm" => "application/wasm".to_string(),

        _ => default_content_type.to_string(),
    }
}
