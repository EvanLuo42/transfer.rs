use reqwest::{Body, Client, Response};
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

use crate::error;

pub async fn put_file(file_path: &str, file_name: &str) -> Response {
    let file = File::open(file_path).await;
    if let Err(_) = file {
        error::file_not_found_error_exit()
    }

    let client = Client::new();
    let res = client
        .put(format!("https://transfer.sh/{}", file_name))
        .body(file_to_body(file.unwrap()))
        .send()
        .await;

    if let Err(_) = res {
        error::request_failed_error_exit()
    }

    res.unwrap()
}

fn file_to_body(file: File) -> Body {
    let stream = FramedRead::new(file, BytesCodec::new());
    let body = Body::wrap_stream(stream);
    body
}