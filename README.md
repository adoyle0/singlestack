# Singlestack

The only stack you'll ever need.

- Nightly
- [Leptos](https://leptos.dev)
- [Axum](github.com/tokio-rs/axum)
- [Compression](https://docs.rs/tower-http/latest/tower_http/compression/)
- [Surrealdb](https://surrealdb.com)
- [Singlestage](https://singlestage.doordesk.net)
- [Tailwind](https://tailwindcss.com)
- [Tauri](https://tauri.app)
- Android

| Server | Port |
|-|-|
| `cargo leptos` | 3000 |
| `trunk` | 3010 |

## Install

```bash
cargo install cargo-generate
```

```bash
cargo generate adoyle0/singlestack
```

## Usage

### Dev build in ssr mode

```bash
cargo install cargo-leptos
```

```bash
./dev ssr
```

### Dev build in csr mode

```bash
cargo install trunk
```

```bash
./dev csr
```

### Dev build native

```bash
cargo install tauri-cli
```

```bash
./dev native
```

### Dev build android

See [https://tauri.app/start/prerequisites/#android](https://tauri.app/start/prerequisites/#android)

```bash
cargo install tauri-cli
```

```bash
cargo tauri android init
```

```bash
./dev android
```
