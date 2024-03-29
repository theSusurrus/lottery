use hyper::server::conn::http1;
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

mod config;
mod http_service;
mod names;

const CONFIG_PATH: &str = "config.toml";

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config = config::LotteryConfig::new(CONFIG_PATH);

    let html_provider = std::sync::Arc::new(names::html::HtmlProvider::new(&config.name_source));

    let service = http_service::LotteryService::new(&http_service::LotteryServiceConfig {
        host_prefix: config.host_dir,
        homepage: config.homepage,
        name_provider: html_provider,
    });

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
