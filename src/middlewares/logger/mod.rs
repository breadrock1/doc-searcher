use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{Error, Result};

use actix_web::body::MessageBody;
use derive_builder::Builder;
use futures::future::LocalBoxFuture;
use futures::FutureExt;
use serde_derive::Serialize;

use std::fmt::Debug;
use std::future::{ready, Ready};
use std::rc::Rc;
use std::task::{Context, Poll};
use std::time::Duration;

#[derive(Builder, Serialize)]
pub struct ErrorRequest {
    pub method: String,
    pub path: String,
    pub agent: String,
    pub status: u16,
}

impl<B> From<&ServiceResponse<B>> for ErrorRequest
where
    B: MessageBody + 'static,
{
    fn from(sr: &ServiceResponse<B>) -> Self {
        let request = sr.request();
        let method = request.method();
        let user_agent = match request.headers().get("User-Agent") {
            None => "Unknown",
            Some(head_val) => head_val.to_str().unwrap_or("Unknown"),
        };

        ErrorRequestBuilder::default()
            .method(method.as_str().to_string())
            .path(request.path().to_string())
            .agent(user_agent.to_string())
            .status(sr.status().as_u16())
            .build()
            .unwrap()
    }
}

pub struct LoggerMiddlewareFactory {
    address: Rc<String>,
    client: Rc<reqwest::Client>,
}

impl LoggerMiddlewareFactory {
    pub fn new(address: &str) -> Self {
        let client = reqwest::ClientBuilder::default()
            .http1_only()
            .build()
            .unwrap();

        LoggerMiddlewareFactory {
            address: Rc::new(address.to_string()),
            client: Rc::new(client),
        }
    }
}

impl<S, B> Transform<S, ServiceRequest> for LoggerMiddlewareFactory
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = LoggerMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(LoggerMiddleware {
            service: Rc::new(service),
            address: self.address.clone(),
            client: self.client.clone(),
        }))
    }
}

pub struct LoggerMiddleware<S> {
    service: Rc<S>,
    address: Rc<String>,
    client: Rc<reqwest::Client>,
}

impl<S, B> Service<ServiceRequest> for LoggerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = S::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();
        let client = self.client.clone();
        let address = self.address.clone();

        async move {
            let service_response: Result<Self::Response, Self::Error> = srv.call(req).await;
            match service_response {
                Err(err) => Err(err),
                Ok(response) => {
                    if !response.status().is_success() {
                        let err_req = ErrorRequest::from(&response);
                        let _ = client
                            .get(address.as_str())
                            .timeout(Duration::from_millis(2))
                            .json(&err_req)
                            .send()
                            .await;
                    }

                    Ok(response)
                }
            }
        }
        .boxed_local()
    }
}
