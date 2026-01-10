use app::*;
use axum::Router;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_axum::{LeptosRoutes, generate_route_list};
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    decompression::RequestDecompressionLayer,
};

mod db;
use db::*;

pub static DATA_DIR: &str = "data";

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db = db_setup().await?;

    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes_with_context(
            &leptos_options,
            routes,
            move || {
                provide_context(db.to_owned());
            },
            {
                let leptos_options = leptos_options.clone();
                move || shell(leptos_options.clone())
            },
        )
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // Support for both request and response compression
    let compression_layer = ServiceBuilder::new()
        .layer(CompressionLayer::new())
        .layer(RequestDecompressionLayer::new());

    // Allow incoming requests from any origin in debug builds to avoid errors when testing things
    // like Tauri builds
    let cors_layer = ServiceBuilder::new().layer(if cfg!(debug_assertions) {
        CorsLayer::new().allow_origin(Any)
    } else {
        CorsLayer::new()
    });

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(
        listener,
        app.layer(compression_layer)
            .layer(cors_layer)
            .into_make_service(),
    )
    .await
    .unwrap();

    Ok(())
}
