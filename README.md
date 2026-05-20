# Hash Tool (Leptos static site)

Rust [Leptos](https://leptos.dev/) app pre-rendered to static HTML (no WASM, no client-side router / SPA). Styled with [Tailwind CSS v4](https://tailwindcss.com/) via [Bun](https://bun.sh/).

## Prerequisites

Optional for `bun run dev` file watching:

```bash
cargo install cargo-watch
```

## Setup

```bash
bun install
bun run css:build
```

## Development

Regenerates HTML on change and serves `target/site`:

```bash
bun run dev
```

Or manually:

```bash
bun run css:watch
cargo run -- --serve
```

Open http://127.0.0.1:3000

## Production build

```bash
bun run build
```

Deploy the contents of `target/site/` to any static host (nginx, GitHub Pages, S3, etc.).

## Preview production build

After building, serve the static output locally:

```bash
bun run build
bun run preview
```

Open http://127.0.0.1:3000

## Project layout

| Path | Purpose |
|------|---------|
| `src/app.rs` | Pages and static route definitions |
| `src/main.rs` | Prerender (`generate_route_list_with_ssg`) and optional `--serve` |
| `style/tailwind.css` | Tailwind entry |
| `style/app.css` | Generated CSS (run `bun run css:build`) |
| `target/site/` | Built static site output |

## Adding routes

Add a `Route` with `ssr=SsrMode::Static(StaticRoute::new())` and a trailing slash in the path when you want `route/index.html` folders (e.g. `StaticSegment("pass_hash")` → `/pass_hash/index.html`).
