use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::{theme::ThemeToggle, ui::PageTitle};

#[component]
pub fn SiteHeader() -> impl IntoView {
    view! {
        <header class="sticky top-0 z-50 border-b border-border/80 bg-card/80 backdrop-blur-md">
            <div class="mx-auto flex max-w-3xl items-center justify-between gap-4 px-4 py-3 sm:px-6">
                <A href="/" attr:class="group flex items-center gap-2 no-underline">
                    <span class="flex h-9 w-9 items-center justify-center rounded-lg bg-primary text-sm font-bold text-primary-foreground shadow-sm">
                        "#"
                    </span>
                    <span class="font-semibold text-foreground group-hover:text-primary transition">
                        "Hash Tool"
                    </span>
                </A>
                <nav class="flex items-center gap-1 sm:gap-2">
                    <NavLink href="/" label="Hashes" />
                    <NavLink href="/pass_hash" label="Pass Hash" />
                    <NavLink href="/encrypt" label="Encrypt" />
                    <ThemeToggle />
                </nav>
            </div>
        </header>
    }
}

#[component]
fn NavLink(href: &'static str, label: &'static str) -> impl IntoView {
    view! {
        <A
            href=href
            attr:class="rounded-lg px-3 py-2 text-sm font-medium text-muted-foreground no-underline transition hover:bg-muted hover:text-foreground aria-[current=page]:bg-accent aria-[current=page]:text-accent-foreground"
        >
            {label}
        </A>
    }
}

#[component]
pub fn SiteFooter() -> impl IntoView {
    view! {
        <footer class="mt-16 border-t border-border py-8">
            <div class="mx-auto flex max-w-3xl justify-center px-4">
                <a
                    href="https://github.com/shubhattin/hash_cryptography_tool"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="inline-flex items-center gap-2 rounded-lg px-3 py-2 text-sm text-muted-foreground no-underline transition hover:bg-muted hover:text-foreground"
                >
                    <svg class="h-5 w-5" viewBox="0 0 24 24" fill="currentColor" aria-hidden="true">
                        <path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0 0 24 12c0-6.63-5.37-12-12-12z"/>
                    </svg>
                    "Source on GitHub"
                </a>
            </div>
        </footer>
    }
}

#[component]
pub fn AppLayout(
    title: &'static str,
    subtitle: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    view! {
        <SiteHeader />
        <div class="mx-auto max-w-3xl px-4 py-8 sm:px-6">
            <PageTitle title=title subtitle=subtitle />
            <div class="space-y-4">
                {children()}
            </div>
        </div>
        <SiteFooter />
    }
}
