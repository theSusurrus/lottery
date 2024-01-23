use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

use hyper::service::Service;
use hyper::{body::Incoming as IncomingBody, Request, Response};
use std::pin::Pin;
use std::future::Future;

mod frontend;
mod pdf;
mod config;

#[derive(Debug, Clone)]
struct LotteryService {
    names: Vec<String>
}

impl Service<Request<IncomingBody>> for LotteryService {
    type Response = Response<Full<Bytes>>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<IncomingBody>) -> Self::Future {
        let mk_generic_response = | s: String | -> Result<Response<Full<Bytes>>, hyper::Error> {
            Ok(Response::builder().body(Full::new(Bytes::from(s))).unwrap())
        };

        let mk_lottery_response = | | -> Result<Response<Full<Bytes>>, hyper::Error> {
            let html = frontend::get_frontend(self.names.clone());
            Ok(Response::builder().body(Full::new(Bytes::from(html))).unwrap())
        };

        let res = match req.uri().path() {
            "/" => mk_generic_response(format!("home")),
            "/names" => mk_generic_response(format!("names = {:?}", self.names)),
            "/lottery" => mk_lottery_response(),
            _ => mk_generic_response("oh no! not found".into()),
        };

        Box::pin(async { res })
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config = config::LotteryConfig::new();

    let service = LotteryService {
        names : pdf::get_names("test.pdf")
    };

    // Bind to the port and listen for incoming TCP connections
    let listener = TcpListener::bind(config.socket).await?;
    println!("Listening on http://{}", config.socket);
    loop {
        // When an incoming TCP connection is received grab a TCP stream for
        // client<->server communication.
        //
        // Note, this is a .await point, this loop will loop forever but is not a busy loop. The
        // .await point allows the Tokio runtime to pull the task off of the thread until the task
        // has work to do. In this case, a connection arrives on the port we are listening on and
        // the task is woken up, at which point the task is then put back on a thread, and is
        // driven forward by the runtime, eventually yielding a TCP stream.
        let (tcp, _) = listener.accept().await?;
        // Use an adapter to access something implementing `tokio::io` traits as if they implement
        // `hyper::rt` IO traits.
        let io = TokioIo::new(tcp);
        let service_clone = service.clone();

        // Spin up a new task in Tokio so we can continue to listen for new TCP connection on the
        // current task without waiting for the processing of the HTTP1 connection we just received
        // to finish
        tokio::task::spawn(async move {
            // Handle the connection from the client using HTTP1 and pass any
            // HTTP requests received on that connection to the `hello` function
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_clone)
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}
