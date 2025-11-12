# Singlestack

The only stack you'll ever need.

- Nightly
- Leptos
- Axum
- Compression
- Surrealdb
- Singlestage
- Tailwind
- Tauri
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
