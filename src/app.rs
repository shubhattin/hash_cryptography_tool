use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    static_routes::StaticRoute,
    StaticSegment, SsrMode,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <MetaTags/>
                <Stylesheet id="leptos" href="/pkg/hash_tool_rs.css"/>
                <Title text="Hash Tool"/>
            </head>
            <body>
                <Router>
                    <main class="min-h-screen bg-slate-950 text-slate-100">
                        <Routes fallback=|| "Page not found.".into_view()>
                            <Route
                                path=StaticSegment("")
                                view=HomePage
                                ssr=SsrMode::Static(StaticRoute::new())
                            />
                        </Routes>
                    </main>
                </Router>
            </body>
        </html>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="mx-auto flex max-w-lg flex-col items-center gap-6 px-6 py-16">
            <h1 class="text-3xl font-bold tracking-tight text-white">
                "Hash Tool"
            </h1>
            <p class="text-center text-slate-400">
                "Static site — Leptos SSR + Tailwind CSS (Bun)"
            </p>
        </div>
    }
}
