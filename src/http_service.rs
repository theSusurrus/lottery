use std::{fs, io};
use std::io::{Read, Write};
use std::ops::{DerefMut};

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

use crate::names::{self, Provider, GenericProvider};

const LOTTERY_PARAM: &str = "lottery";
const NAMES_JSON_PATH: &str = "names.json";

fn get_file(path: String) -> Result<String, io::Error> {
    match fs::File::open(path) {
        Ok(mut file) => {
            let mut contents = Vec::new();
            match file.read_to_end(&mut contents) {
                Ok(_) => {
                    match String::from_utf8(contents) {
                        Ok(string) => Ok(string),
                        Err(error) =>
                            Err(io::Error::new(io::ErrorKind::InvalidData, error)),
                    }
                },
                Err(error) => Err(error),
            }
        },
        Err(error) => Err(error)
    }
}

#[derive(Clone)]
pub struct LotteryServiceConfig {
    pub host_prefix: String,
    pub homepage: String,
    pub name_provider: Arc<dyn names::Provider>,
}

impl std::fmt::Debug for LotteryServiceConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("LotteryServiceConfig")
            .field("host_prefix", &self.host_prefix)
            .field("homepage", &self.homepage)
            .finish()
    }
}

#[derive(Debug, Clone)]
pub struct LotteryService {
    names: Arc<Mutex<Vec<String>>>,
    config: LotteryServiceConfig,
}

impl LotteryService {
    /**
     * Public constructor
     * Gets names from PDF
     */
    pub fn new(config: &LotteryServiceConfig) -> LotteryService {
        LotteryService {
            names : Arc::new(Mutex::new(vec![])),
            config: config.clone(),
        }
    }

    fn update_names(&self) -> Result<(), String> {
        let names_read = self.config.name_provider.get_names();

        match names_read {
            Ok(new_names) => {
                /* Lock names and get mutable reference */
                let mut names_guard = self.names.lock().unwrap();
                let names = names_guard.deref_mut();
        
                /* set new names */
                *names = new_names;

                let json = serde_json::to_string(&names)
                .expect("Cannot serialize names");
    
                let mut file = fs::OpenOptions::new()
                    .read(true)
                    .write(true)
                    .create(true)
                    .open(self.config.host_prefix.clone() + NAMES_JSON_PATH)
                    .expect("Can't open JSON file");
        
                match file.write(&json.as_bytes()) {
                    Ok(bytes) => {
                        println!("Written {bytes} bytes to {NAMES_JSON_PATH}");
                        Ok(())
                    },
                    Err(error) => Err(error.to_string())
                }
            },
            Err(error) => Err(error.to_string())
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
        let mk_generic_response =
            | s: String | -> Result<Response<Full<Bytes>>, hyper::Error> {
            Ok(Response::builder().body(Full::new(Bytes::from(s))).unwrap())
        };

        /* get file and return a Hyper Response containing it */
        let mk_file_response =
            | path: String | -> Result<Response<Full<Bytes>>, hyper::Error> {
            match get_file(path) {
                Ok(html) => {
                    let mut response = Response::builder()
                        .body(
                            Full::new(
                                Bytes::from(
                                    html
                                )
                            )
                        ).unwrap();
                    let headers: &mut hyper::HeaderMap = response.headers_mut();
                    headers.insert(hyper::header::CONTENT_TYPE,
                        "text/html; charset=utf-8".parse().unwrap());

                    Ok(response)
                },
                Err(error) => mk_generic_response(error.to_string())
            }
            
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
            /* lottery query found */
            Some(lottery) => {
                match lottery.as_str() {
                    /* if lottery=new, update the names in self and json */
                    "new" => {
                        match self.update_names() {
                            Ok(_) => (),
                            Err(error) => {
                                return Box::pin(async move {
                                    mk_generic_response(error.to_string())
                                })
                            }
                        }
                    },
                    /* lottery != new, do nothing */
                    _ => ()
                }
            },
            /* lottery query not found, do nothing */
            None => ()
        };

        /* Match a response to a request */
        let res = match req.uri().path() {
            "/" => mk_file_response(self.config.host_prefix.clone() + self.config.homepage.as_str()),
            "/names" => mk_generic_response(format!("names = {:?}", self.names)),
            requested_path => mk_file_response(self.config.host_prefix.clone() + requested_path),
        };

        Box::pin(async { res })
    }
}
