use leptos::prelude::*;

#[component]
pub fn PageTitle(title: &'static str, subtitle: Option<&'static str>) -> impl IntoView {
    view! {
        <div class="mb-8">
            <h1 class="text-2xl font-bold tracking-tight text-foreground sm:text-3xl">{title}</h1>
            {subtitle.map(|s| view! {
                <p class="mt-2 text-sm text-muted-foreground">{s}</p>
            })}
        </div>
    }
}

#[component]
pub fn ToolSection(
    title: &'static str,
    #[prop(optional)] hint: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    view! {
        <details class="group rounded-xl border border-border bg-card shadow-sm open:shadow-md transition-shadow">
            <summary class="flex cursor-pointer list-none items-center justify-between gap-3 px-5 py-4 font-semibold text-card-foreground marker:content-none">
                <span>{title}</span>
                <span class="text-muted-foreground transition group-open:rotate-180">"▾"</span>
            </summary>
            <div class="border-t border-border px-5 py-5">
                {hint.map(|h| view! {
                    <p class="mb-4 text-xs text-muted-foreground">{h}</p>
                })}
                {children()}
            </div>
        </details>
    }
}

#[component]
pub fn FieldLabel(
    label: &'static str,
    #[prop(optional)] hint: Option<&'static str>,
    children: Children,
) -> impl IntoView {
    view! {
        <label class="mb-4 block last:mb-0">
            <span class="mb-1.5 block text-sm font-medium text-foreground">{label}</span>
            {hint.map(|h| view! {
                <span class="mb-2 block text-xs text-muted-foreground">{h}</span>
            })}
            {children()}
        </label>
    }
}

#[component]
pub fn TextInput(
    #[prop(into)] value: RwSignal<String>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(default = false)] password: bool,
) -> impl IntoView {
    let input_type = if password { "password" } else { "text" };
    view! {
        <input
            type=input_type
            class="w-full rounded-lg border border-border bg-background px-3 py-2.5 text-sm text-foreground shadow-sm outline-none transition focus:border-primary focus:ring-2 focus:ring-ring/30"
            placeholder=placeholder.unwrap_or_default()
            prop:value=move || value.get()
            on:input=move |ev| value.set(event_target_value(&ev))
        />
    }
}

#[component]
pub fn TextArea(
    #[prop(into)] value: RwSignal<String>,
    #[prop(default = 4)] rows: u32,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(default = false)] readonly: bool,
) -> impl IntoView {
    view! {
        <textarea
            class="w-full resize-y rounded-lg border border-border bg-background px-3 py-2.5 font-mono text-sm text-foreground shadow-sm outline-none transition focus:border-primary focus:ring-2 focus:ring-ring/30 disabled:opacity-80"
            rows=rows
            placeholder=placeholder.unwrap_or_default()
            readonly=readonly
            prop:value=move || value.get()
            on:input=move |ev| {
                if !readonly {
                    value.set(event_target_value(&ev))
                }
            }
        />
    }
}

#[component]
pub fn PrimaryButton(
    #[prop(into)] label: String,
    #[prop(optional)]
    busy: Option<RwSignal<bool>>,
) -> impl IntoView {
    view! {
        <button
            type="submit"
            class="rounded-lg bg-primary px-4 py-2.5 text-sm font-semibold text-primary-foreground shadow-sm transition hover:opacity-90 disabled:cursor-not-allowed disabled:opacity-50"
            prop:disabled=move || busy.map(|b| b.get()).unwrap_or(false)
        >
            {label}
        </button>
    }
}

#[component]
pub fn SecondaryButton(
    label: &'static str,
    on_click: Callback<()>,
) -> impl IntoView {
    view! {
        <button
            type="button"
            class="rounded-lg border border-border bg-muted px-3 py-2 text-xs font-medium text-foreground transition hover:bg-background"
            on:click=move |_| on_click.run(())
        >
            {label}
        </button>
    }
}

#[component]
pub fn OutputField(#[prop(into)] value: RwSignal<String>) -> impl IntoView {
    let clear = Callback::new(move |_: ()| value.set(String::new()));
    view! {
        <div class="space-y-2">
            <div class="flex justify-end">
                <SecondaryButton label="Clear" on_click=clear />
            </div>
            <TextArea value rows=4 readonly=true />
        </div>
    }
}

#[component]
pub fn StatusBadge(
    valid: RwSignal<Option<bool>>,
    #[prop(optional, into)]
    loading: Option<Signal<bool>>,
) -> impl IntoView {
    let loading = loading.unwrap_or_else(|| Signal::derive(|| false));
    view! {
        <div class="min-h-[2.5rem]">
            {move || loading.get().then(|| view! {
                <span class="inline-flex items-center gap-2 text-sm text-muted-foreground">
                    <span class="h-4 w-4 animate-spin rounded-full border-2 border-primary border-t-transparent"></span>
                    "Working…"
                </span>
            })}
            {move || {
                if loading.get() {
                    return None;
                }
                match valid.get() {
                    Some(true) => Some(view! {
                        <span class="inline-block rounded-lg border border-success/30 bg-success/10 px-3 py-2 text-sm font-medium text-success">
                            "Valid"
                        </span>
                    }.into_any()),
                    Some(false) => Some(view! {
                        <span class="inline-block rounded-lg border border-danger/30 bg-danger/10 px-3 py-2 text-sm font-medium text-danger">
                            "Invalid"
                        </span>
                    }.into_any()),
                    None => None,
                }
            }}
        </div>
    }
}

#[component]
pub fn RadioGroup<T>(
    name: &'static str,
    options: Vec<(&'static str, T)>,
    selected: RwSignal<T>,
) -> impl IntoView
where
    T: Copy + PartialEq + Send + Sync + 'static,
{
    view! {
        <div class="flex flex-wrap gap-3">
            {options.into_iter().enumerate().map(|(i, (label, val))| {
                let is_checked = move || selected.get() == val;
                let input_id = format!("{name}-{i}");
                let label_for = input_id.clone();
                view! {
                    <label
                        for=label_for
                        class="inline-flex cursor-pointer items-center gap-2 rounded-lg border border-border bg-background px-3 py-2 text-sm transition has-checked:border-primary has-checked:bg-accent has-checked:text-accent-foreground"
                    >
                        <input
                            type="radio"
                            class="accent-primary"
                            id=input_id
                            name=name
                            prop:checked=is_checked
                            on:change=move |_| selected.set(val)
                        />
                        {label}
                    </label>
                }
            }).collect_view()}
        </div>
    }
}
