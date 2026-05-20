use std::{env, fs, path::Path};

use axum::Router;
use leptos::config::get_configuration;
use leptos_axum::generate_route_list_with_ssg;
use tower_http::services::ServeDir;

use hash_tool_rs::app::App;

#[tokio::main]
async fn main() {
    env::set_var("LEPTOS_OUTPUT_NAME", "hash_tool_rs");

    let conf = get_configuration(Some("Cargo.toml")).expect("load Leptos config from Cargo.toml");
    let options = conf.leptos_options;

    prepare_site_root(&options);

    let (_, static_generator) = generate_route_list_with_ssg(App);
    static_generator.generate(&options).await;

    sync_assets(&options).expect("sync CSS and public assets into site root");

    if env::args().any(|arg| arg == "--serve") {
        serve_static(&options).await;
    }
}

fn prepare_site_root(options: &leptos::config::LeptosOptions) {
    let site_root = Path::new(options.site_root.as_ref());
    let pkg_dir = site_root.join(options.site_pkg_dir.as_ref());
    if pkg_dir.exists() {
        fs::remove_dir_all(&pkg_dir).ok();
    }
    fs::create_dir_all(&pkg_dir).ok();
}

fn sync_assets(
    options: &leptos::config::LeptosOptions,
) -> Result<(), Box<dyn std::error::Error>> {
    let site_root = Path::new(options.site_root.as_ref());
    let pkg_dir = site_root.join(options.site_pkg_dir.as_ref());
    fs::create_dir_all(&pkg_dir)?;

    let css_src = Path::new("style/app.css");
    if css_src.exists() {
        let css_name = format!("{}.css", options.output_name.as_ref());
        fs::copy(css_src, pkg_dir.join(css_name))?;
    }

    let assets_dir = Path::new("public");
    if assets_dir.is_dir() {
        copy_dir_recursive(assets_dir, site_root)?;
    }

    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), Box<dyn std::error::Error>> {
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let dest = dst.join(entry.file_name());
        if file_type.is_dir() {
            fs::create_dir_all(&dest)?;
            copy_dir_recursive(&entry.path(), &dest)?;
        } else {
            fs::copy(entry.path(), dest)?;
        }
    }
    Ok(())
}

async fn serve_static(options: &leptos::config::LeptosOptions) {
    let site_root = options.site_root.as_ref();
    let addr = options.site_addr;

    let app = Router::new().fallback_service(ServeDir::new(site_root));

    println!("Serving static site at http://{addr}");
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
