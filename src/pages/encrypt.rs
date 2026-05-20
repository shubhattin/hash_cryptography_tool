use leptos::prelude::*;

use crate::components::{
    layout::AppLayout,
    ui::{
        provide_mask_passwords, FieldLabel, MaskPasswordToggle, OutputField, PrimaryButton,
        RadioGroup, TextArea, TextInput, ToolSection,
    },
};
use crate::utils::encrypt::{decrypt_text, encrypt_text};

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum EncMode {
    #[default]
    Encrypt,
    Decrypt,
}

#[component]
pub fn EncryptPage() -> impl IntoView {
    provide_mask_passwords();
    let mode = RwSignal::new(EncMode::Encrypt);
    let text = RwSignal::new(String::new());
    let key = RwSignal::new(String::new());
    let output = RwSignal::new(String::new());
    let error = RwSignal::new(false);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        error.set(false);
        if text.get().is_empty() {
            return;
        }
        match mode.get() {
            EncMode::Encrypt => match encrypt_text(&text.get(), &key.get()) {
                Ok(v) => output.set(v),
                Err(_) => error.set(true),
            },
            EncMode::Decrypt => match decrypt_text(&text.get(), &key.get()) {
                Ok(v) => output.set(v),
                Err(_) => {
                    output.set(String::new());
                    error.set(true);
                }
            },
        }
    };

    view! {
        <AppLayout
            title="Encrypt / Decrypt"
            subtitle=Some("AES-256-GCM with a key derived via SHA-256 from your passphrase. Format: nonce-ciphertext (base64).")
        >
            <MaskPasswordToggle label="Hide passphrase" />
            <ToolSection title="AES-256-GCM">
                <form class="space-y-4" on:submit=on_submit>
                    <FieldLabel label="Mode">
                        <RadioGroup
                            name="enc-mode"
                            options=vec![
                                ("Encrypt", EncMode::Encrypt),
                                ("Decrypt", EncMode::Decrypt),
                            ]
                            selected=mode
                        />
                    </FieldLabel>
                    <FieldLabel label="Text">
                        <TextArea value=text rows=5 placeholder="Plaintext or encrypted payload…".to_string() />
                    </FieldLabel>
                    <FieldLabel label="Passphrase">
                        <TextInput value=key maskable=true placeholder="Secret key…".to_string() />
                    </FieldLabel>
                    <PrimaryButton label="Run".to_string() />
                    {move || {
                        if error.get() {
                            view! {
                                <p class="text-sm font-medium text-danger">
                                    {match mode.get() {
                                        EncMode::Decrypt => "Decryption failed — check key or payload.",
                                        EncMode::Encrypt => "Encryption failed.",
                                    }}
                                </p>
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
        </AppLayout>
    }
}
