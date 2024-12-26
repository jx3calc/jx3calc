## Run

1. install [Rust](https://www.rust-lang.org/tools/install) and Node.js (usually use [nvm](https://github.com/nvm-sh/nvm) or [nvm-windows](https://github.com/coreybutler/nvm-windows) instead).
2. install requirements.
```
pnpm install
cargo install tauri-cli --version "^2.0.0" --locked
```
1. (optional) clone jx3pak and put it in `src/pak/src/jx3pak`.
2. run.
```
cargo tauri dev
```

## Git regulation

**DO NOT** commit to jx3calc organization's repository directly. Insteadly, fork the repository, make changes, and create a Pull Request to the upstream repository.

All Pull Requests will be Squashed and merged. If you want to maintain your own fork, make sure to merge from upstream regularly.