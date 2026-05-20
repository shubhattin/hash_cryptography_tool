use leptos::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum Theme {
    #[default]
    Dark,
    Light,
}

impl Theme {
    pub fn toggle(self) -> Self {
        match self {
            Self::Dark => Self::Light,
            Self::Light => Self::Dark,
        }
    }

    pub fn is_dark(self) -> bool {
        matches!(self, Self::Dark)
    }

    pub fn storage_key() -> &'static str {
        "hash-tool-theme"
    }

    pub fn from_storage(value: &str) -> Self {
        if value == "light" {
            Self::Light
        } else {
            Self::Dark
        }
    }
}

#[derive(Clone, Copy)]
pub struct ThemeCtx(pub RwSignal<Theme>);

pub fn provide_theme() -> ThemeCtx {
    let theme = RwSignal::new(Theme::Dark);
    let ctx = ThemeCtx(theme);
    provide_context(ctx);
    ctx
}

pub fn use_theme() -> ThemeCtx {
    expect_context::<ThemeCtx>()
}

#[cfg(feature = "hydrate")]
fn read_theme_from_storage() -> Option<Theme> {
    let storage = web_sys::window()?.local_storage().ok()??;
    let value = storage.get_item(Theme::storage_key()).ok()??;
    Some(Theme::from_storage(&value))
}

#[cfg(feature = "hydrate")]
fn apply_theme_to_dom(is_dark: bool) {
    if let Some(html) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.document_element())
    {
        let class = if is_dark { "dark" } else { "" };
        let _ = html.set_class_name(class);
    }
    if let Some(storage) = web_sys::window()
        .and_then(|w| w.local_storage().ok())
        .flatten()
    {
        let value = if is_dark { "dark" } else { "light" };
        let _ = storage.set_item(Theme::storage_key(), value);
    }
}

#[component]
pub fn ThemeScript() -> impl IntoView {
    view! {
        <script>
            "(function(){try{var t=localStorage.getItem('hash-tool-theme');var d=t!=='light';document.documentElement.classList.toggle('dark',d);}catch(e){document.documentElement.classList.add('dark');}})();"
        </script>
    }
}

#[component]
pub fn ThemeInit() -> impl IntoView {
    #[cfg(feature = "hydrate")]
    let ThemeCtx(theme) = use_theme();

    #[cfg(feature = "hydrate")]
    {
        Effect::new(move |_| {
            if let Some(stored) = read_theme_from_storage() {
                theme.set(stored);
            }
        });
    }

    view! { <span class="hidden" aria-hidden="true"></span> }
}

#[component]
pub fn ThemeSync() -> impl IntoView {
    let ThemeCtx(theme) = use_theme();

    Effect::new(move |_| {
        let is_dark = theme.get().is_dark();
        #[cfg(feature = "hydrate")]
        apply_theme_to_dom(is_dark);
        #[cfg(not(feature = "hydrate"))]
        let _ = is_dark;
    });

    view! { <span class="hidden" aria-hidden="true"></span> }
}

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let ThemeCtx(theme) = use_theme();

    view! {
        <button
            type="button"
            class="inline-flex h-9 w-9 items-center justify-center rounded-lg border border-border bg-card text-muted-foreground transition hover:bg-muted hover:text-foreground"
            aria-label="Toggle light/dark theme"
            on:click=move |_| theme.update(|t| *t = t.toggle())
        >
            <span class="dark:hidden" aria-hidden="true">"☀"</span>
            <span class="hidden dark:inline" aria-hidden="true">"☽"</span>
        </button>
    }
}
