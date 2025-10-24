use gloo_net::http;

use crux_http::{
    HttpError, Result,
    protocol::{HttpRequest, HttpResponse},
};
use js_sys::{ArrayBuffer, Uint8Array};

#[tracing::instrument(
    skip_all,
    fields(
        http.request.body.size = req.body.len(),
        http.request.method = req.method,
        http.response.status_code = tracing::field::Empty,
        otel.kind = "client",
        span.type = "http",
        url.full = req.url,
    ),
    err(Debug)
)]
pub async fn request(req: &HttpRequest) -> Result<HttpResponse> {
    let span = tracing::Span::current();
    tracing::info!("request start");

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

    let request = if req.body.is_empty() {
        // Build request without body
        request.build().expect("building")
    } else {
        // Create a Uint8Array from our Vec<u8>
        let uint8_array = Uint8Array::new_with_length(req.body.len() as u32);
        uint8_array.copy_from(req.body.as_slice());

        // Convert Uint8Array to ArrayBuffer
        let array_buffer: ArrayBuffer = uint8_array.buffer();

        // Set the body with the ArrayBuffer
        request.body(array_buffer).expect("setting body")
    };

    let response = request
        .send()
        .await
        .map_err(|error| HttpError::Io(error.to_string()))?;
    let body = response
        .binary()
        .await
        .map_err(|error| HttpError::Io(error.to_string()))?;

    span.record("http.response.status_code", response.status());
    if response.status() < 400 {
        tracing::info!("request completed");
    } else if response.status() < 500 {
        tracing::warn!("request invalid");
    } else {
        tracing::error!("request failed");
    }

    Ok(HttpResponse::status(response.status()).body(body).build())
}
