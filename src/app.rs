use leptos::config::LeptosOptions;
use leptos::hydration::{AutoReload, HydrationScripts};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, Meta, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    static_routes::StaticRoute,
    SsrMode, StaticSegment,
};

use crate::components::theme::{provide_theme, ThemeInit, ThemeScript, ThemeSync};
use crate::pages::{encrypt::EncryptPage, hashes::HashesPage, pass_hash::PassHashPage};

#[component]
pub fn Shell(options: LeptosOptions) -> impl IntoView {
    provide_meta_context();

    view! {
        <!DOCTYPE html>
        <html lang="en" class="dark">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <ThemeScript/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options=options/>
                <Meta name="description" content="Hash, encrypt, and password-hash tools in the browser."/>
                <Link rel="icon" href="/favicon.ico"/>
                <Stylesheet id="leptos" href="/pkg/hash_tool_rs.css"/>
                <Title text="Hash Cryptography Tool"/>
                <MetaTags/>
            </head>
            <body class="min-h-screen bg-background font-sans text-foreground antialiased">
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_theme();

    view! {
        <ThemeInit/>
        <ThemeSync/>
        <Router>
                    <Routes fallback=|| view! {
                        <div class="flex min-h-screen items-center justify-center">
                            <p class="text-muted-foreground">"Page not found."</p>
                        </div>
                    }>
                        <Route
                            path=StaticSegment("")
                            view=HashesPage
                            ssr=SsrMode::Static(StaticRoute::new())
                        />
                        <Route
                            path=StaticSegment("pass_hash")
                            view=PassHashPage
                            ssr=SsrMode::Static(StaticRoute::new())
                        />
                        <Route
                            path=StaticSegment("encrypt")
                            view=EncryptPage
                            ssr=SsrMode::Static(StaticRoute::new())
                        />
                    </Routes>
        </Router>
    }
}
