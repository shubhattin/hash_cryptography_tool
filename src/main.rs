#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::config::get_configuration;
    use leptos_axum::{generate_route_list_with_ssg, LeptosRoutes};
    use tower_http::services::ServeDir;

    use hash_tool_rs::app::App;

    let conf = get_configuration(Some("Cargo.toml")).expect("load Leptos config");
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let (routes, static_generator) = generate_route_list_with_ssg(App);
    static_generator.generate(&leptos_options).await;

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, App)
        .fallback_service(
            ServeDir::new(leptos_options.site_root.as_ref())
                .append_index_html_on_directories(true),
        )
        .with_state(leptos_options);

    println!("listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
