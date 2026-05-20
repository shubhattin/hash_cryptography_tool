use leptos::prelude::*;

use crate::components::{
    layout::AppLayout,
    ui::{
        FieldLabel, OutputField, PrimaryButton, RadioGroup, StatusBadge, TextArea, TextInput,
        ToolSection,
    },
};
use crate::utils::{
    hash::{self, DigestAlgorithm},
    pass_hash::{
        self, ArgonVariant, ScryptConfig,
    },
};

#[component]
pub fn PassHashPage() -> impl IntoView {
    view! {
        <AppLayout
            title="Password Hashing"
            subtitle=Some("Salted digests, bcrypt, Argon2, and scrypt — computed in the browser.")
        >
            <SaltedShaSection />
            <SaltedShaVerifySection />
            <BcryptHashSection />
            <BcryptVerifySection />
            <ArgonHashSection />
            <ArgonVerifySection />
            <ScryptHashSection />
            <ScryptVerifySection />
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

fn digest_algo(family: HashFamily, size: HashSize) -> DigestAlgorithm {
    match (family, size) {
        (HashFamily::Sha, HashSize::B256) => DigestAlgorithm::Sha256,
        (HashFamily::Sha, HashSize::B512) => DigestAlgorithm::Sha512,
        (HashFamily::Sha3, HashSize::B256) => DigestAlgorithm::Sha3_256,
        (HashFamily::Sha3, HashSize::B512) => DigestAlgorithm::Sha3_512,
    }
}

#[component]
fn SaltedShaSection() -> impl IntoView {
    let text = RwSignal::new(String::new());
    let output = RwSignal::new(String::new());
    let size = RwSignal::new(HashSize::B256);
    let family = RwSignal::new(HashFamily::Sha);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        if text.get().is_empty() {
            return;
        }
        output.set(hash::hash_password_with_random_salt(
            &text.get(),
            digest_algo(family.get(), size.get()),
        ));
    };

    view! {
        <ToolSection
            title="SHA salted hash"
            hint="Hashes password + random salt, then appends the salt hex to the digest."
        >
            <form class="space-y-4" on:submit=on_submit>
                <FieldLabel label="Algorithm family">
                    <RadioGroup
                        name="psha-family"
                        options=vec![("SHA-2", HashFamily::Sha), ("SHA-3", HashFamily::Sha3)]
                        selected=family
                    />
                </FieldLabel>
                <FieldLabel label="Size">
                    <RadioGroup
                        name="psha-size"
                        options=vec![("256", HashSize::B256), ("512", HashSize::B512)]
                        selected=size
                    />
                </FieldLabel>
                <FieldLabel label="Password">
                    <TextInput value=text password=true />
                </FieldLabel>
                <PrimaryButton label="Hash".to_string() />
                <FieldLabel label="Hash + salt">
                    <OutputField value=output />
                </FieldLabel>
            </form>
        </ToolSection>
    }
}

#[component]
fn SaltedShaVerifySection() -> impl IntoView {
    let text = RwSignal::new(String::new());
    let stored = RwSignal::new(String::new());
    let size = RwSignal::new(HashSize::B256);
    let family = RwSignal::new(HashFamily::Sha);
    let status = RwSignal::new(None::<bool>);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        status.set(None);
        if text.get().is_empty() || stored.get().is_empty() {
            return;
        }
        let algo = digest_algo(family.get(), size.get());
        status.set(Some(
            hash::verify_password_with_salt(&text.get(), &stored.get(), algo)
                .unwrap_or(false),
        ));
    };

    view! {
        <ToolSection title="SHA salted verify">
            <form class="space-y-4" on:submit=on_submit>
                <FieldLabel label="Algorithm family">
                    <RadioGroup
                        name="pshav-family"
                        options=vec![("SHA-2", HashFamily::Sha), ("SHA-3", HashFamily::Sha3)]
                        selected=family
                    />
                </FieldLabel>
                <FieldLabel label="Size">
                    <RadioGroup
                        name="pshav-size"
                        options=vec![("256", HashSize::B256), ("512", HashSize::B512)]
                        selected=size
                    />
                </FieldLabel>
                <FieldLabel label="Password">
                    <TextInput value=text password=true />
                </FieldLabel>
                <FieldLabel label="Stored hash">
                    <TextArea value=stored rows=3 />
                </FieldLabel>
                <PrimaryButton label="Verify".to_string() />
                <StatusBadge valid=status loading=false />
            </form>
        </ToolSection>
    }
}

#[component]
fn BcryptHashSection() -> impl IntoView {
    let text = RwSignal::new(String::new());
    let output = RwSignal::new(String::new());
    let cost = RwSignal::new(11u32);
    let busy = RwSignal::new(false);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        if text.get().is_empty() {
            return;
        }
        busy.set(true);
        let pwd = text.get();
        let c = cost.get();
        let output = output;
        let busy = busy;
        leptos::task::spawn_local(async move {
            let result = pass_hash::bcrypt_hash(&pwd, c).unwrap_or_default();
            output.set(result);
            busy.set(false);
        });
    };

    view! {
        <ToolSection title="Bcrypt hash">
            <form class="space-y-4" on:submit=on_submit>
                <FieldLabel label="Work factor">
                    <input
                        type="range"
                        min="6"
                        max="20"
                        class="w-full accent-primary"
                        prop:value=move || cost.get()
                        on:input=move |ev| {
                            if let Ok(n) = event_target_value(&ev).parse() {
                                cost.set(n);
                            }
                        }
                    />
                    <span class="mt-1 block text-xs text-muted-foreground">
                        {move || format!("Current: {}", cost.get())}
                    </span>
                </FieldLabel>
                <FieldLabel label="Password">
                    <TextInput value=text password=true />
                </FieldLabel>
                <PrimaryButton label="Hash".to_string() busy=busy />
                <FieldLabel label="Bcrypt hash">
                    <OutputField value=output />
                </FieldLabel>
            </form>
        </ToolSection>
    }
}

#[component]
fn BcryptVerifySection() -> impl IntoView {
    let text = RwSignal::new(String::new());
    let stored = RwSignal::new(String::new());
    let status = RwSignal::new(None::<bool>);
    let busy = RwSignal::new(false);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        status.set(None);
        if text.get().is_empty() {
            return;
        }
        if stored.get().len() != 60 {
            status.set(Some(false));
            return;
        }
        busy.set(true);
        let pwd = text.get();
        let hash = stored.get();
        let status = status;
        let busy = busy;
        leptos::task::spawn_local(async move {
            let ok = pass_hash::bcrypt_verify(&pwd, &hash).unwrap_or(false);
            status.set(Some(ok));
            busy.set(false);
        });
    };

    view! {
        <ToolSection title="Bcrypt verify">
            <form class="space-y-4" on:submit=on_submit>
                <FieldLabel label="Password">
                    <TextInput value=text password=true />
                </FieldLabel>
                <FieldLabel label="Bcrypt hash (60 chars)">
                    <TextArea value=stored rows=2 />
                </FieldLabel>
                <PrimaryButton label="Verify".to_string() busy=busy />
                <StatusBadge valid=status loading=Signal::derive(move || busy.get()) />
            </form>
        </ToolSection>
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum ArgonKind {
    #[default]
    Id,
    D,
    I,
}

#[component]
fn ArgonHashSection() -> impl IntoView {
    let text = RwSignal::new(String::new());
    let output = RwSignal::new(String::new());
    let kind = RwSignal::new(ArgonKind::Id);
    let parallelism = RwSignal::new(2u32);
    let iterations = RwSignal::new(128u32);
    let memory = RwSignal::new(512u32);
    let hash_len = RwSignal::new(32u32);
    let busy = RwSignal::new(false);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        if text.get().is_empty() {
            return;
        }
        busy.set(true);
        let pwd = text.get();
        let variant = match kind.get() {
            ArgonKind::Id => ArgonVariant::Id,
            ArgonKind::D => ArgonVariant::D,
            ArgonKind::I => ArgonVariant::I,
        };
        let p = parallelism.get();
        let i = iterations.get();
        let m = memory.get();
        let h = hash_len.get();
        let output = output;
        let busy = busy;
        leptos::task::spawn_local(async move {
            let result = pass_hash::argon2_hash(&pwd, variant, p, i, m, h).unwrap_or_default();
            output.set(result);
            busy.set(false);
        });
    };

    view! {
        <ToolSection title="Argon2 hash">
            <form class="space-y-4" on:submit=on_submit>
                <FieldLabel label="Variant">
                    <RadioGroup
                        name="argon-kind"
                        options=vec![
                            ("Argon2id", ArgonKind::Id),
                            ("Argon2d", ArgonKind::D),
                            ("Argon2i", ArgonKind::I),
                        ]
                        selected=kind
                    />
                </FieldLabel>
                <div class="grid gap-4 sm:grid-cols-2">
                    <FieldLabel label="Parallelism">
                        <NumberInput value=parallelism min=1 max=16 />
                    </FieldLabel>
                    <FieldLabel label="Iterations">
                        <NumberInput value=iterations min=1 max=65536 />
                    </FieldLabel>
                    <FieldLabel label="Memory (KiB)">
                        <NumberInput value=memory min=8 max=1048576 />
                    </FieldLabel>
                    <FieldLabel label="Hash length (bytes)">
                        <NumberInput value=hash_len min=16 max=64 />
                    </FieldLabel>
                </div>
                <FieldLabel label="Password">
                    <TextInput value=text password=true />
                </FieldLabel>
                <PrimaryButton label="Hash".to_string() busy=busy />
                <FieldLabel label="PHC hash">
                    <OutputField value=output />
                </FieldLabel>
            </form>
        </ToolSection>
    }
}

#[component]
fn ArgonVerifySection() -> impl IntoView {
    let text = RwSignal::new(String::new());
    let stored = RwSignal::new(String::new());
    let status = RwSignal::new(None::<bool>);
    let busy = RwSignal::new(false);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        status.set(None);
        if text.get().is_empty() || stored.get().len() < 50 {
            status.set(Some(false));
            return;
        }
        busy.set(true);
        let pwd = text.get();
        let hash = stored.get();
        let status = status;
        let busy = busy;
        leptos::task::spawn_local(async move {
            let ok = pass_hash::argon2_verify(&pwd, &hash).unwrap_or(false);
            status.set(Some(ok));
            busy.set(false);
        });
    };

    view! {
        <ToolSection title="Argon2 verify">
            <form class="space-y-4" on:submit=on_submit>
                <FieldLabel label="Password">
                    <TextInput value=text password=true />
                </FieldLabel>
                <FieldLabel label="PHC hash">
                    <TextArea value=stored rows=3 />
                </FieldLabel>
                <PrimaryButton label="Verify".to_string() busy=busy />
                <StatusBadge valid=status loading=Signal::derive(move || busy.get()) />
            </form>
        </ToolSection>
    }
}

#[component]
fn ScryptHashSection() -> impl IntoView {
    let text = RwSignal::new(String::new());
    let output = RwSignal::new(String::new());
    let cost = RwSignal::new(8u32);
    let block = RwSignal::new(8u32);
    let parallel = RwSignal::new(1u32);
    let len = RwSignal::new(64u32);
    let busy = RwSignal::new(false);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        if text.get().is_empty() {
            return;
        }
        busy.set(true);
        let pwd = text.get();
        let cfg = ScryptConfig {
            cost_factor: cost.get(),
            block_size: block.get(),
            parallelism: parallel.get(),
            hash_length: len.get() as usize,
        };
        let output = output;
        let busy = busy;
        leptos::task::spawn_local(async move {
            let result = pass_hash::scrypt_hash(&pwd, cfg).unwrap_or_default();
            output.set(result);
            busy.set(false);
        });
    };

    view! {
        <ToolSection title="Scrypt hash" hint="Format: salt_hex:hash_hex">
            <form class="space-y-4" on:submit=on_submit>
                <div class="grid gap-4 sm:grid-cols-2">
                    <FieldLabel label="Cost factor (N)">
                        <ScryptCostSelect value=cost />
                    </FieldLabel>
                    <FieldLabel label="Block size (r)">
                        <NumberInput value=block min=1 max=32 />
                    </FieldLabel>
                    <FieldLabel label="Parallelism (p)">
                        <NumberInput value=parallel min=1 max=16 />
                    </FieldLabel>
                    <FieldLabel label="Hash length">
                        <NumberInput value=len min=8 max=512 />
                    </FieldLabel>
                </div>
                <FieldLabel label="Password">
                    <TextInput value=text password=true />
                </FieldLabel>
                <PrimaryButton label="Hash".to_string() busy=busy />
                <FieldLabel label="Scrypt hash">
                    <OutputField value=output />
                </FieldLabel>
            </form>
        </ToolSection>
    }
}

#[component]
fn ScryptVerifySection() -> impl IntoView {
    let text = RwSignal::new(String::new());
    let stored = RwSignal::new(String::new());
    let cost = RwSignal::new(8u32);
    let block = RwSignal::new(8u32);
    let parallel = RwSignal::new(1u32);
    let len = RwSignal::new(64u32);
    let status = RwSignal::new(None::<bool>);
    let busy = RwSignal::new(false);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        status.set(None);
        if text.get().is_empty() || stored.get().is_empty() {
            return;
        }
        busy.set(true);
        let pwd = text.get();
        let hash = stored.get();
        let cfg = ScryptConfig {
            cost_factor: cost.get(),
            block_size: block.get(),
            parallelism: parallel.get(),
            hash_length: len.get() as usize,
        };
        let status = status;
        let busy = busy;
        leptos::task::spawn_local(async move {
            let ok = pass_hash::scrypt_verify(&pwd, &hash, cfg).unwrap_or(false);
            status.set(Some(ok));
            busy.set(false);
        });
    };

    view! {
        <ToolSection title="Scrypt verify">
            <form class="space-y-4" on:submit=on_submit>
                <div class="grid gap-4 sm:grid-cols-2">
                    <FieldLabel label="Cost factor (N)">
                        <ScryptCostSelect value=cost />
                    </FieldLabel>
                    <FieldLabel label="Block size (r)">
                        <NumberInput value=block min=1 max=32 />
                    </FieldLabel>
                    <FieldLabel label="Parallelism (p)">
                        <NumberInput value=parallel min=1 max=16 />
                    </FieldLabel>
                    <FieldLabel label="Hash length">
                        <NumberInput value=len min=8 max=512 />
                    </FieldLabel>
                </div>
                <FieldLabel label="Password">
                    <TextInput value=text password=true />
                </FieldLabel>
                <FieldLabel label="Stored hash">
                    <TextArea value=stored rows=3 />
                </FieldLabel>
                <PrimaryButton label="Verify".to_string() busy=busy />
                <StatusBadge valid=status loading=Signal::derive(move || busy.get()) />
            </form>
        </ToolSection>
    }
}

#[component]
fn NumberInput(
    value: RwSignal<u32>,
    min: u32,
    max: u32,
) -> impl IntoView {
    view! {
        <input
            type="number"
            min=min
            max=max
            class="w-full rounded-lg border border-border bg-background px-3 py-2 text-sm"
            prop:value=move || value.get()
            on:input=move |ev| {
                if let Ok(n) = event_target_value(&ev).parse::<u32>() {
                    value.set(n.clamp(min, max));
                }
            }
        />
    }
}

#[component]
fn ScryptCostSelect(value: RwSignal<u32>) -> impl IntoView {
    let costs: Vec<u32> = (2..=16).map(|i| 2u32.pow(i)).collect();
    view! {
        <select
            class="w-full rounded-lg border border-border bg-background px-3 py-2 text-sm"
            on:change=move |ev| {
                if let Ok(n) = event_target_value(&ev).parse() {
                    value.set(n);
                }
            }
        >
            {costs.into_iter().map(|c| {
                let is_sel = move || value.get() == c;
                view! {
                    <option value=c.to_string() prop:selected=is_sel>{c}</option>
                }
            }).collect_view()}
        </select>
    }
}
