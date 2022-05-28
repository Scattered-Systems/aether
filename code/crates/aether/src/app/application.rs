use axum;
use crate::{settings::Settings, app::endpoints};
use http::header;
use serde::{Deserialize, Serialize};
use tower_http::{
    compression::CompressionLayer,
    propagate_header::PropagateHeaderLayer,
    sensitive_headers::SetSensitiveHeadersLayer,
    trace,
};

#[derive(Clone, Debug)]
pub struct Application {
    pub address: std::net::SocketAddr,
    pub context: Context
}

impl Application {
    pub async fn new() -> Self {
        let settings = match Settings::new() {
            Ok(value) => value,
            Err(err) => panic!("ConfigurationError: {:#?}", err)
        };
        Logger::setup(&settings);

        let host = [0, 0, 0, 0];
        let port = settings.server.port;

        let address: std::net::SocketAddr = std::net::SocketAddr::from((host, port));
        let context = Context::new(settings.clone());

        let client = axum::Router::new()
            .merge(endpoints::base::create_route())
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
            .layer(axum::Extension(context.clone()));

        println!("{}", settings.server);

        axum::Server::bind(&address)
            .serve(client.into_make_service())
            .await
            .expect("Failed to start server");

        Self { address, context }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Context {
    pub settings: Settings,
}

impl Context {
    pub fn new(settings: Settings) -> Self {
        Self {
            settings
        }
    }
}

pub struct Logger;

impl Logger {
    pub fn setup(settings: &Settings) {
        if std::env::var_os("RUST_LOG").is_none() {
            let level = settings.logger.level.as_str();
            let env = format!("dapi={},tower_http=debug", level);

            std::env::set_var("RUST_LOG", env);
        }

        tracing_subscriber::fmt::init();
    }
}