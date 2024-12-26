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

The title of the Pull Request and the corresponding commit message should be the same, and follow this format: `type: 内容`. Additionally, the commit message should add ` (#th)` at the end.

1. `type` should be one of these: `init`, `feat`, `fix`, `doc`, `refactor` and `other`.
2. `:` should be half-width (`:`, not `：`.), and a space should follow it.
3. `内容` should write in Chinese.
4. If `内容` is more than one sentence, end with `.` or `。`, otherwise end without it directly.
5. Punctuation in `内容` should be consistent in width, DO NOT mix full-width and half-width. Make sure to add spaces in the appropriate places when using half-width punctuation.
6. DO NOT add spaces around SINGLE word in `内容`, but DO add around MULTIPLE words.
7. ` (#th)` depends on the serial number of the Pull Request. Pay attention to the space before the brackets.