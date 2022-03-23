use axum::response::IntoResponse;
use axum::{extract::Extension, response::Html, routing::get, Router};
use prometheus::Registry;
use std::net::SocketAddr;
use tokio::signal;

use srs_exporter::{parse_config, MetricCollector, NacosClient, CURRENT_VERSION, DEFAULT_CONFIG};

#[tokio::main]
async fn main() {
    // 0. setup tracing log
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "srs_exporter=info,tokio=info,tower_http=info")
    }
    tracing_subscriber::fmt::init();

    // 1. treat first arg as config file location & parse config
    let f = match std::env::args().nth(1) {
        Some(f) => f,
        None => DEFAULT_CONFIG.to_string(),
    };
    let toml_config = parse_config(f).unwrap();
    let app_config = toml_config.app.clone();
    let srs_config = toml_config.srs.clone();

    // 2. spawn a task to check srs and report to nacos
    tokio::spawn(async move {
        let nacos_client = NacosClient::new(&toml_config);
        match nacos_client.register_service().await {
            Ok(_) => tracing::info!("Nacos service registration succeed"),
            Err(e) => tracing::error!("{}", e),
        }
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            // process every two seconds
            match nacos_client.clone().ping_pong().await {
                Ok(_) => tracing::info!("Nacos service ping pong succeed"),
                Err(e) => tracing::error!("{}", e),
            }
        }
    });

    // 3. create shared_state which will be MetricCollector
    let shared_collector = MetricCollector::new(Registry::new(), srs_config);

    // 4. http server
    let addr = SocketAddr::from(([0, 0, 0, 0], app_config.port));

    tracing::info!(
        "Srs Exporter will listen on {}, Current Version is {}",
        addr,
        CURRENT_VERSION
    );
    let app = Router::new()
        .route("/", get(root))
        // .route(
        //     "/slow",
        //     get(|| async {
        //         tokio::time::sleep(Duration::from_secs(1)).await;
        //     }),
        // )
        .route("/metrics", get(collect))
        .layer(Extension(shared_collector));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn root() -> Html<&'static str> {
    Html(
        "<div>
        <p>Hello, This is SRS Exporter!</p>
        <p>Please check on <a href=\"/metrics\">/metrics</a>.</p>
        </div>",
    )
}

async fn collect(Extension(mc): Extension<MetricCollector>) -> impl IntoResponse {
    mc.collect().await
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("signal received, starting graceful shutdown");
}
