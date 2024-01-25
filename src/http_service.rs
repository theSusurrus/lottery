use std::{fs, io};
use std::io::Write;
use std::ops::{Deref};

use hyper::service::Service;
use hyper::{body::Incoming as IncomingBody, Request, Response};
use serde_json;
use std::pin::Pin;
use std::future::Future;
use http_body_util::Full;
use hyper::body::Bytes;
use url;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::pdf;

const LOTTERY_PARAM: &str = "lottery";
const NAMES_JSON_PATH: &str = "names.json";

fn get_file(path: String) -> String {
    match fs::read_to_string(path) {
        Ok(html) => html,
        Err(error) => error.to_string()
    }
}

#[derive(Debug, Clone)]
pub struct LotteryService {
    names: Arc<Mutex<Vec<String>>>,
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
            names : Arc::new(Mutex::new(vec![])),
            host_prefix : "host/".to_string(),
            homepage : "home.html".to_string(),
        }
    }

    /**
     * Get immutable copy of shared names vector
     */
    fn get_names(&self) -> Vec<String> {
        let mut names_guard = self.names.lock().unwrap();
        names_guard.deref().clone()
    }

    fn update_names(&self) {
        pdf::get_names()
    }

    /**
     * Serialize the name vector into JSON and save it to file for reading by the frontend
     */
    fn save_names_to_file(&self) -> io::Result<usize> {
        let names = self.get_names();

        let json = serde_json::to_string(&names)
            .expect("Cannot serialize names");

        let mut file = fs::File::open(NAMES_JSON_PATH)
            .expect("Can't open JSON file");

        file.write(&json.as_bytes())
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
        let mk_generic_response =
            | s: String | -> Result<Response<Full<Bytes>>, hyper::Error> {
            Ok(Response::builder().body(Full::new(Bytes::from(s))).unwrap())
        };

        /* get file and return a Hyper Response containing it */
        let mk_file_response =
            | path: String | -> Result<Response<Full<Bytes>>, hyper::Error> {
            let html = get_file(path);
            Ok(Response::builder().body(Full::new(Bytes::from(html))).unwrap())
        };

        let params: HashMap<String, String> = req
            .uri()
            .query()
            .map(|v| {
                url::form_urlencoded::parse(v.as_bytes())
                    .into_owned()
                    .collect()
            })
            .unwrap_or_else(HashMap::new);

        println!("{:?} {}\n\tparams={:?}", req.method(), req.uri(), params);
        
        match params.get(LOTTERY_PARAM) {
            Some(lottery) => {
                match lottery.as_str() {
                    "new" => self.save_names_to_file(),
                    _ => ()
                }
            },
            None => ()
        };

        /* Match a response to a request */
        let res = match req.uri().path() {
            "/" => mk_file_response(self.host_prefix.clone() + self.homepage.as_str()),
            "/names" => mk_generic_response(format!("names = {:?}", self.names.lock().unwrap().deref())),
            requested_path => mk_file_response(self.host_prefix.clone() + requested_path),
        };

        Box::pin(async { res })
    }
}
