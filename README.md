## Run

1. install [Rust](https://www.rust-lang.org/tools/install) and Node.js (usually use [nvm](https://github.com/nvm-sh/nvm) or [nvm-windows](https://github.com/coreybutler/nvm-windows) instead).
2. install requirements.
```shell
pnpm install
cargo install tauri-cli --version "^2.0.0" --locked
```
3. (optional) clone jx3pak and put it in `src/pak/src/jx3pak`.
4. run.
```shell
cargo tauri dev
```

Every member in workspace of `Cargo.toml` is a Rust crate. You can also run them as a binary use `cargo run --bin crate-name`, for example:

```shell
cargo run --bin pak # run the `pak` crate
```

## Git regulation

**DO NOT** commit to jx3calc organization's repository directly. Insteadly, fork the repository, make changes, and create a Pull Request to the upstream repository.

All Pull Requests will be Squashed and merged. If you want to maintain your own fork, make sure to rebase from upstream regularly.