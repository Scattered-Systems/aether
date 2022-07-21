/*
   Appellation: interface <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use utils::*;

use crate::{Configuration, Context, Logger};
use std::net::SocketAddr;
use tower_http::{
    compression::CompressionLayer, propagate_header::PropagateHeaderLayer,
    sensitive_headers::SetSensitiveHeadersLayer, trace,
};

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Interface {
    pub address: SocketAddr,
    pub context: Context,
}

impl Interface {
    pub fn new() -> Self {
        let cnf = Configuration::new().ok().unwrap();

        Logger::setup(&cnf);

        let host: [u8; 4] = extract_list_from_string(cnf.server.host.clone(), ',')
            .try_into()
            .ok()
            .unwrap();
        let port = cnf.server.port;

        let address: SocketAddr = SocketAddr::from((host, port));
        let context = Context::new(cnf.clone());

        Self { address, context }
    }

    pub async fn run(&mut self) {
        create_server(self.address.clone(), self.context.clone())
            .await
            .expect("Server Error")
    }
}

impl std::fmt::Display for Interface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "View the application locally at http://localhost:{}",
            self.context.configuration.server.port
        )
    }
}

mod utils {
    use super::*;
    use axum::{routing::IntoMakeService, Router, Server};
    use hyper::server::conn::AddrIncoming;

    pub type AxumServer = Server<AddrIncoming, IntoMakeService<Router>>;

    pub fn create_server(address: SocketAddr, context: Context) -> AxumServer {
        let client = Router::new()
            .merge(crate::index::create_route())
            .layer(
                trace::TraceLayer::new_for_http()
                    .make_span_with(trace::DefaultMakeSpan::new().include_headers(true))
                    .on_request(trace::DefaultOnRequest::new().level(tracing::Level::INFO))
                    .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
            )
            .layer(SetSensitiveHeadersLayer::new(std::iter::once(
                http::header::AUTHORIZATION,
            )))
            .layer(CompressionLayer::new())
            .layer(PropagateHeaderLayer::new(
                http::header::HeaderName::from_static("x-request-id"),
            ))
            .layer(axum::Extension(context.clone()));
        Server::bind(&address).serve(client.into_make_service())
    }

    pub fn extract_list_from_string<T>(string: String, breakpoint: char) -> Vec<T>
        where
            T: Clone + std::str::FromStr,
            <T as std::str::FromStr>::Err: std::fmt::Debug,
    {
        let exclude: &[char] = &[' ', ',', '[', ']', '.'];
        let trimmed: &str = &string.trim_matches(exclude);
        trimmed
            .split(breakpoint)
            .map(|i| i.trim_matches(exclude).parse::<T>().unwrap())
            .collect()
    }
}
