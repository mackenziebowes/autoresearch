#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use std::path::PathBuf;

    use axum::Router;
    use axum::routing::get;
    use leptos::prelude::*;
    use leptos_axum::{LeptosRoutes, generate_route_list};
    use viewer::app::{App, shell};

    tracing_subscriber::fmt::init();

    let root = std::env::var_os("AUTORESEARCH_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../research"));
    viewer::server::init(root)?;

    let conf = get_configuration(None)?;
    let options = conf.leptos_options;
    let addr = options.site_addr;
    let routes = generate_route_list(App);

    let app = Router::new()
        .route("/files/{slug}/{*path}", get(viewer::server::serve_file))
        .leptos_routes(&options, routes, {
            let options = options.clone();
            move || shell(options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(options);

    tracing::info!("listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

#[cfg(not(feature = "ssr"))]
fn main() {
    // The binary is server-only; the browser bundle is built from lib.rs.
}
