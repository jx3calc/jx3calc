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

**禁止** 直接提交至 jx3calc 组织的仓库. 作为替代, fork 此仓库, 进行改动后向 upstream 发起一个 Pull Request.

所有的 Pull Request 都会被 squash and merge, 即便它们只包含一个提交. 这意味着在您的 Pull Request 被合并后, upstream 将会与您的仓库产生冲突, 请确保自行解决这一问题.

Pull Request 的标题与其生成的 commit 标题内容相同, 并遵循同样的格式: `类型: 内容`. 不同的是, commit 的标题会在最后附带 ` (#th)`, 以标记其对应的 Pull Request 序号.

1. `类型` 应当是以下项之一: `init`, `feat`, `fix`, `doc`, `refactor`, `ci`. (若后续确实需要添加其他项, 应同步更新本文档.)
2. `类型` 与 `内容` 之间的 `:` 应当为半角, 并且其后跟随一个空格.
3. `内容` 应当使用中文书写.
4. `内容` 超出一个句子时, 使用句号(全角或半角均可)结尾. 否则, 直接结尾, 不要添加句号.
5. `内容` 中的标点符号应当统一宽度(统一为全角或半角). 如果使用半角标点符号, 确保在合适的位置添加空格.
6. `内容` 中出现英文时, 如果是单个单词, 不要在其与其周围的中文字符间添加空格; 如果是多个单词, 则应当添加.
7. ` (#th)` 取决于对应的 Pull Request 序号. 注意括号前应当与 `内容` 之间留出空格.

## Code regulation

#### 导入与导出

##### 导入规则

如果一个文件导入了同名目录下的其他文件作为模块 (如在 `a.rs` 中使用 `mod b;`, 其中 b 位于 `a/b.rs`), 那么这个文件中不应当包含除了 **模块导入** , **模块定义** 与 **名称导入** 以外的任何内容.

> 模块导入: `mod xxx;`
> 模块定义: `mod xxx{}`
> 名称导入: `use xxx`;

如果确实需要定义一些通用内容以供子模块使用, 请定义一个子模块 `_mod`, 在其中定义这些通用内容, 并定义可见性.

通过将父模块中的代码移动至一个特殊的 `_mod` 兄弟模块, 可以使代码的逻辑在可见性方面更清晰.

##### 导出规则

所有的导出 (`pub`) 都必须声明作用域, 如 `pub(crate)`, `pub(super)` 等.

#### 文件头部

在有逻辑代码的文件中, 不应当包含模块导入 (`mod` 语句). (见上文: 导入规则)

对于名称导入 (`use` 语句):

1. 导入时, 应当将以下两部分分开, 中间用空行隔开:
   - 第一部分: `super`, `crate`, (直接使用本仓库中代码的)自己的第三方 crate
   - 第二部分: 第三方 crate
2. 导入时应当尽可能细化至模块的下一级 (函数, 结构体等), 除非一个模块下有大量内容需要导入 (如果这个模块属于自己的第三方 crate, 此时应当重新组织文件结构).
   > 如应导入 `pak::{tab_init, tab_et}` 并直接调用 `tab_init` 与 `tab_get`, 而不是导入 `pak` 然后调用 `pak::tab_init` 与 `pak::tab_get`.
3. 所有使用的第三方 crate 都 **必须** 导入. 禁止不进行导入而直接使用 crate 名称的方式使用第三方 crate.
4. **不要** 在文件头部导入 `std` crate 下的内容. 尽可能使用它们的完整路径. 如有必要, 可以在函数体的顶部导入它们, 特别是在不导入就无法直接使用某些方法的情况下.
   > 如应使用 `std::io::Result`, 而不是 `use std::io::Result` 后直接使用 `Result`
