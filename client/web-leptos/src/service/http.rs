use gloo_net::http;

use crux_http::{
    HttpError, Result,
    protocol::{HttpRequest, HttpResponse},
};
use js_sys::{ArrayBuffer, Uint8Array};

pub async fn request(req: &HttpRequest) -> Result<HttpResponse> {
    let request = match req.method.as_str() {
        "GET" => http::Request::get(&req.url),
        "POST" => http::Request::post(&req.url),
        "PUT" => http::Request::put(&req.url),
        "DELETE" => http::Request::delete(&req.url),
        "PATCH" => http::Request::patch(&req.url),
        _ => panic!("not yet handling this method"),
    };

    let request = req.headers.iter().fold(request, |req, header| {
        req.header(&header.name, &header.value)
    });

    // Create a Uint8Array from our Vec<u8>
    let uint8_array = Uint8Array::new_with_length(req.body.len() as u32);
    uint8_array.copy_from(req.body.as_slice());

    // Convert Uint8Array to ArrayBuffer
    let array_buffer: ArrayBuffer = uint8_array.buffer();

    // Set the body with the ArrayBuffer
    let request = request.body(array_buffer).expect("setting body");

    let response = request
        .send()
        .await
        .map_err(|error| HttpError::Io(error.to_string()))?;
    let body = response
        .binary()
        .await
        .map_err(|error| HttpError::Io(error.to_string()))?;

    Ok(HttpResponse::status(response.status()).body(body).build())
}
