use leptos::prelude::*;

use crate::components::{
    layout::AppLayout,
    ui::{
        FieldLabel, OutputField, PrimaryButton, RadioGroup, ReadonlyOutput, TextArea, ToolSection,
    },
};
use crate::utils::{
    codec::{
        decode_base64, encode_base64, generate_alphanumeric, generate_uuid_v4, generate_uuid_v6,
    },
    hash::{self, DigestAlgorithm},
};

#[component]
pub fn HashesPage() -> impl IntoView {
    view! {
        <AppLayout
            title="Cryptography Tools"
            subtitle=Some("Digest hashes, encoding, salts, and random identifiers.")
        >
            <DigestHashSection />
            <Base64Section />
            <SaltSection />
            <UuidSection />
            <AlphanumericSection />
        </AppLayout>
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum HashSize {
    #[default]
    B256,
    B512,
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum HashFamily {
    #[default]
    Sha,
    Sha3,
}

#[component]
fn DigestHashSection() -> impl IntoView {
    let text = RwSignal::new(String::new());
    let output = RwSignal::new(String::new());
    let size = RwSignal::new(HashSize::B256);
    let family = RwSignal::new(HashFamily::Sha);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        if text.get().is_empty() {
            return;
        }
        let algo = match (family.get(), size.get()) {
            (HashFamily::Sha, HashSize::B256) => DigestAlgorithm::Sha256,
            (HashFamily::Sha, HashSize::B512) => DigestAlgorithm::Sha512,
            (HashFamily::Sha3, HashSize::B256) => DigestAlgorithm::Sha3_256,
            (HashFamily::Sha3, HashSize::B512) => DigestAlgorithm::Sha3_512,
        };
        output.set(hash::hash_text(&text.get(), algo));
    };

    view! {
        <ToolSection
            title="Hash"
            hint="SHA-256/512 and SHA3-256/512. Output length is 64 hex chars (256-bit) or 128 (512-bit)."
        >
            <form class="space-y-4" on:submit=on_submit>
                <div class="space-y-3">
                    <FieldLabel label="Algorithm family">
                        <RadioGroup
                            name="hash-family"
                            options=vec![("SHA-2", HashFamily::Sha), ("SHA-3", HashFamily::Sha3)]
                            selected=family
                        />
                    </FieldLabel>
                    <FieldLabel label="Output size">
                        <RadioGroup
                            name="hash-size"
                            options=vec![("256-bit", HashSize::B256), ("512-bit", HashSize::B512)]
                            selected=size
                        />
                    </FieldLabel>
                </div>
                <FieldLabel label="Text to hash">
                    <TextArea value=text rows=4 placeholder="Enter text…".to_string() />
                </FieldLabel>
                <PrimaryButton label="Hash".to_string() />
                <FieldLabel label="Digest (hex)">
                    <OutputField value=output />
                </FieldLabel>
            </form>
        </ToolSection>
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum Base64Mode {
    #[default]
    Encode,
    Decode,
}

#[component]
fn Base64Section() -> impl IntoView {
    let text = RwSignal::new(String::new());
    let output = RwSignal::new(String::new());
    let mode = RwSignal::new(Base64Mode::Encode);
    let error = RwSignal::new(false);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        error.set(false);
        if text.get().is_empty() {
            return;
        }
        match mode.get() {
            Base64Mode::Encode => output.set(encode_base64(&text.get())),
            Base64Mode::Decode => match decode_base64(&text.get()) {
                Ok(s) => output.set(s),
                Err(_) => {
                    output.set(String::new());
                    error.set(true);
                }
            },
        }
    };

    view! {
        <ToolSection title="Base64" hint="Encode or decode UTF-8 text.">
            <form class="space-y-4" on:submit=on_submit>
                <FieldLabel label="Mode">
                    <RadioGroup
                        name="b64-mode"
                        options=vec![("Encode", Base64Mode::Encode), ("Decode", Base64Mode::Decode)]
                        selected=mode
                    />
                </FieldLabel>
                <FieldLabel label="Input">
                    <TextArea value=text rows=4 />
                </FieldLabel>
                <PrimaryButton label="Run".to_string() />
                {move || {
                    if error.get() {
                        view! {
                            <p class="text-sm font-medium text-danger">"Invalid Base64 input."</p>
                        }.into_any()
                    } else {
                        view! {
                            <FieldLabel label="Output">
                                <OutputField value=output />
                            </FieldLabel>
                        }.into_any()
                    }
                }}
            </form>
        </ToolSection>
    }
}

#[component]
fn SaltSection() -> impl IntoView {
    let salt = RwSignal::new(String::new());

    view! {
        <ToolSection title="Salt" hint="16 random bytes as 32 hex characters.">
            <div class="space-y-4">
                <button
                    type="button"
                    class="rounded-lg bg-primary px-4 py-2.5 text-sm font-semibold text-primary-foreground shadow-sm transition hover:opacity-90"
                    on:click=move |_| salt.set(hash::generate_salt_hex())
                >
                    "Generate salt"
                </button>
                <FieldLabel label="Salt">
                    <ReadonlyOutput value=salt rows=2 />
                </FieldLabel>
            </div>
        </ToolSection>
    }
}

#[component]
fn UuidSection() -> impl IntoView {
    let uuid = RwSignal::new(String::new());

    view! {
        <ToolSection
            title="UUID"
            hint="v4: random. v6: time-ordered and sortable by creation time (RFC 9562)."
        >
            <div class="space-y-4">
                <div class="flex flex-wrap gap-2">
                    <button
                        type="button"
                        class="rounded-lg bg-primary px-4 py-2.5 text-sm font-semibold text-primary-foreground shadow-sm transition hover:opacity-90"
                        on:click=move |_| uuid.set(generate_uuid_v4())
                    >
                        "Generate v4"
                    </button>
                    <button
                        type="button"
                        class="rounded-lg border border-border bg-muted px-4 py-2.5 text-sm font-semibold text-foreground transition hover:bg-background"
                        on:click=move |_| uuid.set(generate_uuid_v6())
                    >
                        "Generate v6"
                    </button>
                </div>
                <FieldLabel label="UUID">
                    <ReadonlyOutput value=uuid rows=1 />
                </FieldLabel>
            </div>
        </ToolSection>
    }
}

#[component]
fn AlphanumericSection() -> impl IntoView {
    let code = RwSignal::new(String::new());
    let length = RwSignal::new(32u32);

    view! {
        <ToolSection title="Alphanumeric code">
            <div class="space-y-4">
                <FieldLabel label="Length">
                    <input
                        type="number"
                        min="1"
                        max="512"
                        class="w-32 rounded-lg border border-border bg-background px-3 py-2 text-sm"
                        prop:value=move || length.get()
                        on:input=move |ev| {
                            if let Ok(n) = event_target_value(&ev).parse::<u32>() {
                                length.set(n.clamp(1, 512));
                            }
                        }
                    />
                </FieldLabel>
                <button
                    type="button"
                    class="rounded-lg bg-primary px-4 py-2.5 text-sm font-semibold text-primary-foreground shadow-sm transition hover:opacity-90"
                    on:click=move |_| code.set(generate_alphanumeric(length.get() as usize))
                >
                    "Generate code"
                </button>
                <FieldLabel label="Result">
                    <ReadonlyOutput value=code rows=2 />
                </FieldLabel>
            </div>
        </ToolSection>
    }
}
