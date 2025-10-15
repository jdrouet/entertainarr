use std::sync::Arc;

use crux_core::render::RenderOperation;
use crux_http::{
    HttpError,
    protocol::{HttpHeader, HttpRequest, HttpResponse},
};
use entertainarr_client_core::{Effect, ViewModel};
use reqwest::Method;

use crate::view::Render;

pub mod view;

pub struct Application {
    core: Arc<crux_core::Core<entertainarr_client_core::Application>>,
    http_client: reqwest::blocking::Client,
    sender: std::sync::mpsc::SyncSender<Effect>,
    receiver: std::sync::mpsc::Receiver<Effect>,
}

impl Application {
    pub fn new() -> Self {
        let (sender, receiver) = std::sync::mpsc::sync_channel::<Effect>(1024);
        let _ = sender.send(Effect::Render(crux_core::Request {
            operation: RenderOperation,
            handle: crux_core::RequestHandle::Never,
        }));
        Self {
            core: Arc::new(crux_core::Core::new()),
            http_client: reqwest::blocking::Client::new(),
            sender,
            receiver,
        }
    }

    fn perform_http_request(&self, req: &HttpRequest) -> Result<HttpResponse, HttpError> {
        let method = Method::from_bytes(req.method.as_bytes())
            .map_err(|err| HttpError::Url(err.to_string()))?;

        let headers = req.headers.iter().map(|header| {
            let name = reqwest::header::HeaderName::from_bytes(header.name.as_bytes())
                .expect("Invalid header name");
            let value = reqwest::header::HeaderValue::from_bytes(header.value.as_bytes())
                .expect("Invalid header value");

            (name, value)
        });

        let request = self
            .http_client
            .request(method, &req.url)
            .headers(headers.collect::<reqwest::header::HeaderMap<_>>())
            .body(req.body.clone())
            .build()
            .map_err(|err| HttpError::Url(err.to_string()))?;

        let response = self
            .http_client
            .execute(request)
            .map_err(|err| HttpError::Io(err.to_string()))?;

        let headers = response
            .headers()
            .iter()
            .map(|(name, value)| {
                value
                    .to_str()
                    .map(|value| HttpHeader {
                        name: name.to_string(),
                        value: value.to_string(),
                    })
                    .map_err(|err| HttpError::Io(err.to_string()))
            })
            .collect::<Result<Vec<HttpHeader>, HttpError>>()?;

        Ok(HttpResponse {
            status: response.status().as_u16(),
            headers,
            body: response
                .bytes()
                .map_err(|err| HttpError::Io(err.to_string()))?
                .to_vec(),
        })
    }

    fn handle_http(&self, mut req: crux_core::Request<HttpRequest>) -> anyhow::Result<()> {
        println!("% Calling server...");
        let res = self
            .perform_http_request(&req.operation)
            .inspect(|res| match res.status {
                0..400 => println!("% Request completed"),
                400..500 => eprintln!("% Request failed, invalid input"),
                500.. => eprintln!("% Request failed, server error"),
            })
            .inspect_err(|err| eprintln!("% Request failed: {err:?}"));

        for effect in self.core.resolve(&mut req, res.into())? {
            let _ = self.sender.send(effect);
        }

        Ok(())
    }

    fn handle_render(&self) -> anyhow::Result<()> {
        let ViewModel { view } = self.core.view();
        let event = view.render()?;
        for effect in self.core.process_event(event) {
            let _ = self.sender.send(effect);
        }
        Ok(())
    }

    pub fn run(self) -> anyhow::Result<()> {
        while let Ok(effect) = self.receiver.recv() {
            match effect {
                Effect::Http(req) => {
                    self.handle_http(req)?;
                }
                Effect::Render(_) => {
                    self.handle_render()?;
                }
            }
        }
        Ok(())
    }
}
