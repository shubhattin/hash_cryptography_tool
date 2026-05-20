# Hash Cryptography Tool

Browser-side hashing, encryption, and password tools. Live: [hash.shubhattin.in](https://hash.shubhattin.in)

**Stack:** [Leptos](https://leptos.dev/) · [Tailwind CSS v4](https://tailwindcss.com/) · [Bun](https://bun.sh/) · Rust

All crypto runs in the browser — nothing is sent to a server. UTF-8 supported throughout.

## Features

**Hashes** (`/`)
- SHA-256, SHA-512, SHA3-256, SHA3-512 digests
- Base64 encode / decode
- Random salt (16 bytes, hex)
- UUID v4 and v6 generators
- Random alphanumeric codes

**Password hashing** (`/pass_hash`)
- Salted SHA-256 / SHA-512 / SHA3-256 / SHA3-512 (hash + verify)
- Bcrypt (hash + verify)
- Argon2id / Argon2d / Argon2i (hash + verify)
- Scrypt (hash + verify), including **Better Auth** preset

**Encrypt / decrypt** (`/encrypt`)
- AES-256-GCM with passphrase (SHA-256 key derivation)

## Commands

```bash
bun install          # setup
bun run dev            # watch + rebuild → target/site
bun run build          # production build
bun run preview        # serve target/site at :3000
cargo test             # Rust tests
```

## History

- [SvelteKit](https://kit.svelte.dev/) + [PicoCSS](https://picocss.com/) until [this commit](https://github.com/shubhattin/hash_cryptography_tool/tree/8aab7cf9615c2189a3b8268c069280da9b49dc85)
- [FastAPI](https://fastapi.tiangolo.com/) + [htmx](https://htmx.org/) + PicoCSS until [this commit](https://github.com/shubhattin/hash_cryptography_tool/tree/694834ecd6fd9f0a283d8f4992a6e57b345953da)
