#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::config::get_configuration;
    use leptos::prelude::*;
    use leptos_axum::{
        file_and_error_handler, generate_route_list_with_exclusions_and_ssg_and_context,
        LeptosRoutes,
    };

    use hash_tool_rs::app::Shell;

    let conf = get_configuration(Some("Cargo.toml")).expect("load Leptos config");
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;

    // SSG must render the full Shell (with HydrationScripts), not App alone.
    let (routes, static_generator) =
        generate_route_list_with_exclusions_and_ssg_and_context(
            {
                let leptos_options = leptos_options.clone();
                move || {
                    let options = leptos_options.clone();
                    view! { <Shell options/> }
                }
            },
            None,
            || {},
        );
    static_generator.generate(&leptos_options).await;

    if std::env::args().any(|arg| arg == "--ssg-only") {
        return;
    }

    let app = Router::new()
        .leptos_routes(
            &leptos_options,
            routes,
            {
                let leptos_options = leptos_options.clone();
                move || {
                    let options = leptos_options.clone();
                    view! { <Shell options/> }
                }
            },
        )
        .fallback(file_and_error_handler(|options| view! { <Shell options/> }))
        .with_state(leptos_options);

    println!("listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {}
