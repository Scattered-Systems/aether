use axum;

use http::header;
use tower_http::{
    compression::CompressionLayer,
    propagate_header::PropagateHeaderLayer,
    sensitive_headers::SetSensitiveHeadersLayer,
    trace,
};
use web3;

use crate::{
    core::{context::Context, logger::Logger, settings::Settings}
};

#[derive(Clone, Debug)]
pub struct Connections {
    pub chain: web3::Web3<web3::transports::Http>,
}

impl Connections {
    pub fn new(settings: Settings) -> Result<Self, web3::Error> {
        let endpoint: String = settings.provider.endpoint;
        let transport = web3::transports::Http::new(endpoint.as_str())?;
        let chain = web3::Web3::new(transport);
        Ok(
            Self {
                chain
            }
        )
    }
}

#[derive(Clone, Debug)]
pub struct Interface {
    pub address: std::net::SocketAddr,
    pub chain: web3::Web3<web3::transports::Http>,
}

impl Interface {
    pub async fn new() -> Self {
        let settings = match Settings::new() {
            Ok(value) => value,
            Err(err) => panic!("ConfigurationError: {:#?}", err)
        };

        Logger::setup(&settings);

        let host = settings.server.host;
        let port = settings.server.port;
        let address: std::net::SocketAddr = std::net::SocketAddr::from((host, port));

        let connections = match Connections::new(settings.clone()) {
            Ok(value) => value,
            Err(err) => panic!("Connection Error: {:#?}", err)
        };
        let chain = connections.chain;
        let context = Context::new(settings.clone());

        let client = axum::Router::new()
            .merge(crate::api::endpoints::base::create_route())
            .layer(
                trace::TraceLayer::new_for_http()
                    .make_span_with(
                        trace::DefaultMakeSpan::new().include_headers(true)
                    )
                    .on_request(
                        trace::DefaultOnRequest::new().level(tracing::Level::INFO)
                    )
                    .on_response(
                        trace::DefaultOnResponse::new().level(tracing::Level::INFO)
                    ),
            )
            .layer(
                SetSensitiveHeadersLayer::new(
                    std::iter::once(
                        header::AUTHORIZATION
                    )
                )
            )
            .layer(
                CompressionLayer::new()
            )
            .layer(
                PropagateHeaderLayer::new(
                    header::HeaderName::from_static(
                        "x-request-id"
                    )
                )
            )
            .layer(axum::Extension(context));

        println!("{}", settings.server);

        axum::Server::bind(&address)
            .serve(client.into_make_service())
            .await
            .expect("Failed to start server");

        Self { address, chain }
    }
}