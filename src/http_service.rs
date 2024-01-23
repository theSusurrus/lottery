use std::fs;

use hyper::service::Service;
use hyper::{body::Incoming as IncomingBody, Request, Response};
use std::pin::Pin;
use std::future::Future;
use http_body_util::Full;
use hyper::body::Bytes;

use crate::pdf;

fn get_file(path: String) -> String {
    match fs::read_to_string(path) {
        Ok(html) => html,
        Err(error) => error.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct LotteryService {
    names: Vec<String>,
    host_prefix: String,
    homepage: String,
}

impl LotteryService {
    /**
     * Public constructor
     * Gets names from PDF
     */
    pub fn new() -> LotteryService {
        LotteryService {
            names : pdf::get_names("test.pdf"),
            host_prefix : "host/".to_string(),
            homepage : "home.html".to_string(),
        }
    }
}

/**
 * Implement a Hyper Service trait for LotteryService
 * It can then be passed to Hyper for calling.
 */
impl Service<Request<IncomingBody>> for LotteryService {
    type Response = Response<Full<Bytes>>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<IncomingBody>) -> Self::Future {
        /* make a plaintext reponse */
        let mk_generic_response = | s: String | -> Result<Response<Full<Bytes>>, hyper::Error> {
            Ok(Response::builder().body(Full::new(Bytes::from(s))).unwrap())
        };

        /* get file and return a Hyper Response containing it */
        let mk_file_response = | path: String | -> Result<Response<Full<Bytes>>, hyper::Error> {
            let html = get_file(path);
            Ok(Response::builder().body(Full::new(Bytes::from(html))).unwrap())
        };

        /* Match a response to a request */
        let res = match req.uri().path() {
            "/" => mk_file_response(self.host_prefix.clone() + self.homepage.as_str()),
            "/names" => mk_generic_response(format!("names = {:?}", self.names)),
            requested_path => mk_file_response(self.host_prefix.clone() + requested_path),
        };

        Box::pin(async { res })
    }
}
