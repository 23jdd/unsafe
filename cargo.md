`Cargo.toml` 可以理解成 Rust 项目的 **`go.mod + package.json + 一部分构建配置`**。Cargo 官方把它叫做 **manifest**，主要配置包信息、依赖、target、feature、workspace、编译 profile 等。([Rust Documentation][1])

一个比较完整、实用的例子：

```toml
[package]
name = "myapp"
version = "0.1.0"
edition = "2024"
rust-version = "1.85"

authors = ["Sam <sam@example.com>"]
description = "My Rust application"
license = "MIT"
readme = "README.md"
repository = "https://github.com/user/myapp"
homepage = "https://github.com/user/myapp"

# 是否允许发布
publish = false


# =========================
# 普通依赖
# =========================

[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }

# 本地依赖
my-lib = { path = "../my-lib" }

# Git 依赖
# my-lib = { git = "https://github.com/user/my-lib" }

# 可选依赖
tracing = { version = "0.1", optional = true }


# =========================
# 开发依赖
# cargo test / example / bench
# =========================

[dev-dependencies]
pretty_assertions = "1"


# =========================
# build.rs 使用的依赖
# =========================

[build-dependencies]
cc = "1"


# =========================
# Feature
# =========================

[features]

# cargo build 默认开启
default = ["logging"]

logging = ["dep:tracing"]

full = [
    "logging",
]


# =========================
# library
# =========================

[lib]
name = "myapp"
path = "src/lib.rs"


# =========================
# binary
# =========================

[[bin]]
name = "myapp"
path = "src/main.rs"


# =========================
# Example
# =========================

[[example]]
name = "basic"
path = "examples/basic.rs"


# =========================
# Test
# =========================

[[test]]
name = "integration"
path = "tests/integration.rs"


# =========================
# Benchmark
# =========================

[[bench]]
name = "benchmark"
path = "benches/benchmark.rs"
harness = false


# =========================
# Release profile
# =========================

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = true
panic = "abort"


[profile.dev]
opt-level = 0
debug = true
```

### 1. `[package]`

最基本：

```toml
[package]
name = "hello"
version = "0.1.0"
edition = "2024"
```

常用字段：

```toml
[package]
name = "hello"
version = "1.2.3"

edition = "2024"

# 最低支持 Rust 版本
rust-version = "1.85"

authors = ["Alice <alice@example.com>"]

description = "A useful library"

license = "MIT"

readme = "README.md"

repository = "https://github.com/alice/hello"

homepage = "https://hello.example.com"

documentation = "https://docs.rs/hello"

keywords = ["cli", "tool"]

categories = ["command-line-utilities"]
```

如果不想发布到 registry：

```toml
publish = false
```

如果项目有：

```text
build.rs
```

Cargo 默认会自动发现。

也可以：

```toml
build = "build.rs"
```

或者关闭：

```toml
build = false
```

([Rust Documentation][1])

### 2. `[dependencies]`

最简单：

```toml
[dependencies]
serde = "1"
tokio = "1"
```

基本等价于：

```toml
serde = { version = "1" }
```

指定 feature：

```toml
serde = {
    version = "1",
    features = ["derive"]
}
```

一般实际写成一行：

```toml
serde = { version = "1", features = ["derive"] }
```

关闭依赖自己的默认 feature：

```toml
flate2 = {
    version = "1",
    default-features = false,
    features = ["zlib-rs"]
}
```

Cargo feature 是**累加式**的：如果依赖图中其他 crate 又启用了这个 feature，它最终仍可能被启用。([Rust Documentation][2])

### 3. path dependency

类似 Go：

```go
replace example.com/x => ../x
```

Rust：

```toml
[dependencies]
mylib = { path = "../mylib" }
```

也可以同时写版本：

```toml
mylib = {
    version = "1.0",
    path = "../mylib"
}
```

本地开发使用 `path`，publish 后其他人使用 registry 版本。

### 4. Git dependency

```toml
[dependencies]
mylib = {
    git = "https://github.com/user/mylib"
}
```

指定 branch：

```toml
mylib = {
    git = "https://github.com/user/mylib",
    branch = "dev"
}
```

指定 tag：

```toml
mylib = {
    git = "https://github.com/user/mylib",
    tag = "v1.2.0"
}
```

指定 commit：

```toml
mylib = {
    git = "https://github.com/user/mylib",
    rev = "abc123"
}
```

### 5. optional dependency

```toml
[dependencies]
tracing = {
    version = "0.1",
    optional = true
}
```

然后：

```toml
[features]
logging = ["dep:tracing"]
```

启用：

```bash
cargo build --features logging
```

代码：

```rust
#[cfg(feature = "logging")]
fn init_logging() {
    println!("logging enabled");
}
```

推荐写：

```toml
logging = ["dep:tracing"]
```

而不是旧式：

```toml
logging = ["tracing"]
```

因为 `dep:` 明确表示“启用这个可选依赖”。([Rust Documentation][2])

### 6. `[features]`

例如：

```toml
[features]
default = ["json"]

json = ["dep:serde", "dep:serde_json"]

http = ["dep:reqwest"]

full = [
    "json",
    "http"
]
```

命令：

```bash
cargo build
```

开启：

```text
default
json
```

关闭 default：

```bash
cargo build --no-default-features
```

开启指定：

```bash
cargo build --features http
```

多个：

```bash
cargo build --features "json http"
```

全部：

```bash
cargo build --all-features
```

([Rust Documentation][2])

### 7. 三种 dependency

Cargo 有三个非常重要的依赖区：

```toml
[dependencies]
```

正常代码：

```rust
use serde::Serialize;
```

---

测试、example、benchmark：

```toml
[dev-dependencies]
criterion = "0.7"
```

---

`build.rs`：

```toml
[build-dependencies]
cc = "1"
```

例如：

```rust
// build.rs

fn main() {
    cc::Build::new()
        .file("src/foo.c")
        .compile("foo");
}
```

这些 dependency section 是 Cargo manifest 的标准组成部分。([Rust Documentation][1])

### 8. 平台特定依赖

比如 Windows：

```toml
[target.'cfg(windows)'.dependencies]
windows = "0.61"
```

Linux：

```toml
[target.'cfg(target_os = "linux")'.dependencies]
nix = "0.30"
```

macOS：

```toml
[target.'cfg(target_os = "macos")'.dependencies]
objc2 = "0.6"
```

也可以按 architecture：

```toml
[target.'cfg(target_arch = "x86_64")'.dependencies]
```

这非常适合写跨平台库。

### 9. `[lib]`

默认：

```text
src/lib.rs
```

所以通常根本不用写：

```toml
[lib]
```

自定义：

```toml
[lib]
name = "foo"
path = "src/foo.rs"
```

比如生成动态库：

```toml
[lib]
crate-type = ["cdylib"]
```

常见类型：

```toml
crate-type = [
    "lib",
    "rlib",
    "dylib",
    "cdylib",
    "staticlib"
]
```

例如 Rust 给 C 调：

```toml
[lib]
crate-type = ["cdylib"]
```

或者：

```toml
crate-type = ["staticlib"]
```

### 10. `[[bin]]`

默认：

```text
src/main.rs
```

自动生成：

```text
target/debug/myapp
```

多个 binary：

```toml
[[bin]]
name = "server"
path = "src/bin/server.rs"

[[bin]]
name = "client"
path = "src/bin/client.rs"
```

然后：

```bash
cargo run --bin server
```

```bash
cargo run --bin client
```

其实 Cargo 还能自动识别：

```text
src/bin/server.rs
src/bin/client.rs
```

所以很多时候不需要配置。

### 11. `[[example]]`

默认目录：

```text
examples/
```

例如：

```text
examples/basic.rs
```

运行：

```bash
cargo run --example basic
```

通常也不用手动写 `Cargo.toml`。

需要自定义才写：

```toml
[[example]]
name = "demo"
path = "examples/custom.rs"
```

### 12. `[[test]]`

Cargo 默认自动找：

```text
tests/*.rs
```

例如：

```text
tests/api.rs
```

运行：

```bash
cargo test
```

自定义：

```toml
[[test]]
name = "api"
path = "tests/api.rs"
```

### 13. `[[bench]]`

默认：

```text
benches/
```

比如 Criterion：

```toml
[dev-dependencies]
criterion = "0.7"

[[bench]]
name = "my_benchmark"
harness = false
```

目录：

```text
benches/my_benchmark.rs
```

运行：

```bash
cargo bench
```

### 14. `[profile.dev]`

Cargo 的编译优化配置。

开发：

```toml
[profile.dev]
opt-level = 0
debug = true
```

release：

```toml
[profile.release]
opt-level = 3
debug = false
```

常见优化：

```toml
[profile.release]
opt-level = 3

# Link Time Optimization
lto = true

# 更好的优化，但编译更慢
codegen-units = 1

# 去掉符号
strip = true

# panic 时直接 abort
panic = "abort"
```

如果目标是**小 binary**：

```toml
[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

### 15. Workspace

这个和你熟悉的：

```text
go.work
```

很像。

目录：

```text
project/
├── Cargo.toml
├── Cargo.lock
└── crates/
    ├── server/
    │   └── Cargo.toml
    ├── client/
    │   └── Cargo.toml
    └── core/
        └── Cargo.toml
```

根目录：

```toml
[workspace]
members = [
    "crates/server",
    "crates/client",
    "crates/core",
]

resolver = "3"
```

也可以：

```toml
[workspace]
members = ["crates/*"]
resolver = "3"
```

Workspace 成员共享一个根目录 `Cargo.lock` 和默认的 `target/`。([Rust Documentation][3])

### 16. Workspace 统一 dependency

这是大型 Rust 项目特别常见的写法：

```toml
[workspace]
members = ["crates/*"]
resolver = "3"

[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
anyhow = "1"
```

然后：

```text
crates/server/Cargo.toml
```

```toml
[dependencies]
serde.workspace = true
tokio.workspace = true
anyhow.workspace = true
```

也可以：

```toml
serde = { workspace = true }
```

这样就不用每个 crate 都：

```toml
serde = "1"
tokio = "1"
```

非常推荐。Workspace dependency inheritance 是 Cargo 官方支持的机制。([Rust Documentation][3])

### 17. Workspace 继承 package 信息

根目录：

```toml
[workspace.package]
version = "0.1.0"
edition = "2024"
license = "MIT"
authors = ["Sam"]
repository = "https://github.com/user/project"
```

子 crate：

```toml
[package]
name = "core"

version.workspace = true
edition.workspace = true
license.workspace = true
authors.workspace = true
repository.workspace = true
```

大型项目很好用。

### 18. `[lints]`

可以在 Cargo 中统一 lint：

```toml
[lints.rust]
unsafe_code = "forbid"
unused = "warn"
```

比如你现在正在学 unsafe Rust，如果项目想完全禁止 unsafe：

```toml
[lints.rust]
unsafe_code = "forbid"
```

等价于项目级：

```rust
#![forbid(unsafe_code)]
```

Workspace 也可以统一：

```toml
[workspace.lints.rust]
unsafe_code = "forbid"
```

子项目：

```toml
[lints]
workspace = true
```

Cargo workspace lint inheritance 已经是正式支持的机制。([Rust Documentation][1])

### 19. `[patch.crates-io]`

这个很像 Go 的 `replace`。

例如本来：

```toml
[dependencies]
foo = "1"
```

但是你想暂时测试本地版本：

```toml
[patch.crates-io]
foo = { path = "../foo" }
```

现在：

```text
crates.io/foo
        ↓
../foo
```

整个 workspace 都会用这个版本。

### 20. `[package.metadata]`

Cargo 自己不解释它，主要给工具用：

```toml
[package.metadata.mytool]
foo = "bar"
enabled = true
```

你的程序/工具可以读取。

Workspace：

```toml
[workspace.metadata.mytool]
foo = "bar"
```

Cargo 会允许工具在这些区域存自己的配置。([Rust Documentation][3])

---

所以你可以先把 `Cargo.toml` 记成这个结构：

```toml
[package]
# 我是谁

[dependencies]
# 正常依赖

[dev-dependencies]
# test/example/bench

[build-dependencies]
# build.rs

[features]
# 条件功能

[lib]
# library

[[bin]]
# executable

[[example]]
# example

[[test]]
# integration test

[[bench]]
# benchmark

[profile.dev]
# debug 编译配置

[profile.release]
# release 编译配置

[workspace]
# 多 crate 项目

[workspace.dependencies]
# workspace 公共依赖

[lints]
# lint

[patch.crates-io]
# 替换依赖
```

如果从 **Go 的角度**来记：

| Rust                | Go                              |
| ------------------- | ------------------------------- |
| `Cargo.toml`        | `go.mod` + 部分 build config      |
| `[package]`         | `module` 的项目信息                  |
| `[dependencies]`    | `require`                       |
| `{ path = "../x" }` | `replace => ../x`               |
| `[patch.crates-io]` | `replace`                       |
| `[workspace]`       | `go.work`                       |
| `[features]`        | Go 没有直接对应物                      |
| `[profile.release]` | `go build` 编译参数                 |
| `Cargo.lock`        | `go.sum` 不完全对应，更像 lockfile      |
| `cargo add`         | `go get`                        |
| `cargo update`      | `go get -u` / dependency update |
| `cargo build`       | `go build`                      |

你目前刚开始系统学 Rust 的话，**先掌握 `[package]`、`[dependencies]`、`[features]`、`[workspace]`、`[profile]` 这 5 个就够了**；其余基本都是碰到场景再学。

[1]: https://doc.rust-lang.org/cargo/reference/manifest.html?utm_source=chatgpt.com "The Manifest Format - The Cargo Book"
[2]: https://doc.rust-lang.org/cargo/reference/features.html?utm_source=chatgpt.com "Features - The Cargo Book"
[3]: https://doc.rust-lang.org/cargo/reference/workspaces.html?utm_source=chatgpt.com "Workspaces - The Cargo Book"
