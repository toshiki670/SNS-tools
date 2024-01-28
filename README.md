# Tauri + Yew

This template should help get you started developing with Tauri and Yew.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

## Preparation

```text
yarn
```

```text
cargo install cargo-make
cargo make prepare_development
```

## To get started run

```text
yarn tauri dev
```

## Reference

<https://heroicons.com>

## Update

### Backend

```text
cd src-tauri
rustup update
```

[crates](https://crates.io) で最新バージョンを確認

```test
cargo update
```

### Frontend

```test
yarn upgrade --latest

# or

yarn upgrade-interactive
```

## DB

```bash
# Create DB
$ sqlx db create --database-url="sqlite://database.db"

# Create Migration file
$ sqlx migrate add -r ${name}

# Migrate run
$ sqlx migrate run  --database-url="sqlite://database.db"

# Migrate revert
$ sqlx migrate revert  --database-url="sqlite://database.db"

# Prepare for `sqlx::query_as!`.
$ cargo make prepare_sqlx
```

## Note

### Cache clean

```text
// Rust
// prepare: cargo install cargo-cache
$ cargo cache -a

// Yarn
$ yarn cache clean
```

### About RSPC

Core と Frontend 型を共有するライブラリ
インストールを試みた Commit ID は、以下の通り

```text
commit c56e1d66f69a1d40fa577a9264fe89d00f7a6999 (HEAD -> Use-tauri-url-client)
```
