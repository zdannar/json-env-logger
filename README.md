<h1 align="center">
  json env logger
</h1>

<p align="center">
   A simple structured JSON logger for Rust.  
   <i>This crate is a rename and republish of the original json_env_logger.  The author abandoned it.</i>
</p>

<div align="center">
  <a alt="GitHub Actions" href="https://github.com/zdannar/json-env-logger/actions">
    <img src="https://github.com/zdannar/json-env-logger/workflows/Main/badge.svg"/>
  </a>
  <a alt="crates.io" href="https://crates.io/crates/json-env-logger">
    <img src="https://img.shields.io/crates/v/json_env_logger.svg?logo=rust"/>
  </a>
  <a alt="docs.rs" href="http://docs.rs/json-env-logger">
    <img src="https://docs.rs/json_env_logger/badge.svg"/>
  </a>
  <a alt="latest docs" href="https://zdannar.github.io/json-env-logger">
   <img src="https://img.shields.io/badge/docs-latest-green.svg"/>
  </a>
  <a alt="license" href="LICENSE">
    <img src="https://img.shields.io/badge/license-MIT-brightgreen.svg"/>
  </a>
</div>

<br />

## install

Add the following to your `Cargo.toml` file

```toml
[dependencies]
json_env_logger2 = "0.2"
```

## usage

### tl;dr

When you run

```bash
$ RUST_LOG=trace cargo -q run \
  --example default \
  --features iso-timestamps
```

You will get

```json
{"level":"TRACE","ts":"2020-07-06T03:41:57.831Z","msg":"I am a trace","task_id":567,"thread_id":"12"}
{"level":"DEBUG","ts":"2020-07-06T03:41:57.832Z","msg":"I am a debug","foo":1}
{"level":"INFO","ts":"2020-07-06T03:41:57.832Z","msg":"I am an info"}
{"level":"WARN","ts":"2020-07-06T03:41:57.832Z","msg":"I am a warning"}
{"level":"ERROR","ts":"2020-07-06T03:41:57.832Z","msg":"I am an error"}
```

### basic use

Like `env_logger`, call `init` before you start your logging engines.

```rust
fn main() {
    json_env_logger::init();

    log::info!("👋")
}
```

Run your program with `RUST_LOG=info your/bin`

### adding more structure

The `log` crate is working its way towards adding first class interfaces for structured fields
in its macros. Rather then reinvent that functionality, this crate embraces it.  `json_env_logger` will serialize these key-value pairs when present. Sadly the `log` crate
doesn't expose these macro interfaces quite yet. ...But `kv-log-macro` does!

```toml
[dependencies]
json_env_logger = "0.1"
kv-log-macro = "1.0"
```

```rust
// the macros exported in this crate are
// log compatible and will one day be merged 
// so save your future self the toil of find and replace
use kv_log_macro as log;

fn main() {
    json_env_logger::init();

    log::info!("👋", { size: "medium", age: 42 })
}
```

> ⭐ These structured fields are currently limited for now to values of type `u64`, `i64`, `f64`, `bool`, `char` or `str`

Run your program with `RUST_LOG=info your/bin`

### iso-timestamps

By default json_env_logger uses unix epoch time for timestamps. You might prefer
ISO-8601 timestamps instead. You can swap implementations with by enabling the `iso-timestamps` feature

```toml
[dependencies]
json_env_logger = { version = "0.1", features = ["iso-timestamps"] }
```

```rust
fn main() {
    json_env_logger::init();

    log::info!("👋")
}
```

Run your program with `RUST_LOG=info your/bin`

### panic visibility

When panics are unavoidable, you can register a panic hook that serializes the panic to json before logging it with `error!`

```rust
fn main() {
    json_env_logger::init();
    json_env_logger::panic_hook();

    panic!("whoooops!")
}
```

Run your program with `RUST_LOG=info your/bin`

> ⭐ You can also serialize backtraces by enabling the `backtrace` cargo feature

## faq

<details><summary>Why do I need structured logging?</summary>
<p>

Maybe you don't. ...But maybe you do! You might if you run applications in production in an environment whose log aggregation does useful things
for you if you emit json logs such as

  - structured field based filters, an alternative to artisanal regex queries
  - aggregation statistics 
  - alert automation
  - anomaly detection
  - basically anything a computer can do for you when it's logs are structured in a machine readable format 

</p>
</details>
&nbsp;

 <details><summary>What use case does json_env_logger target?</summary>
<p>

Most folks on the Rust logging market start out with [`log`](https://crates.io/crates/log). They soon find they need configurable logging so they move to [`env_logger`](https://crates.io/crates/env_logger). Sometimes they want `env_logger` but pretty logging for host local application so they move to [`pretty_env_logger`](https://crates.io/crates/pretty_env_logger) of if you like [`emoji-logger`](https://crates.io/crates/emoji-logger).

In other cases you want to run applications in a cloud service that rewards you for emitting logs in JSON format. That's use case this targets, those coming from `env_logger` but would like to leverage build in JSON log parsing and discovery options their cloud provided offers for free.
</p>
</details>
&nbsp;
 <details><summary>Does one of these already exist?</summary>
<p>

A few actually. Like many crates in the Rust ecosystem, they are all good. Picking a dependency is a dance of picking your tradeoffs given an application's goals and needs.

There's [`slog`](https://github.com/slog-rs/slog), an entire ecosystem of logging for Rust. It's strength is that its highly configurable. It's drawback is that it's highly configurable interface can get in the way of simple cases where you just want to emit structured logs in json without a lot of ceremony.

Here's an example directly from its [docs](https://docs.rs/slog-json/2.3.0/slog_json/)

```rust
#[macro_use]
extern crate slog;
extern crate slog_json;

use slog::Drain;
use std::sync::Mutex;

fn main() {
    let root = slog::Logger::root(
        Mutex::new(slog_json::Json::default(std::io::stderr())).map(slog::Fuse),
        o!("version" => env!("CARGO_PKG_VERSION"))
    );
}
```

vs

```rust
fn main() {
    json_env_logger::init();
}
```

`slog`'s encouraged programming model is to pass loggers instances around as arguments. This is a good practice and allows for simple context propagation, but comes at the expense of being a much different programming model that others using the standard `log` crate have written code against so you may find yourself having to rewrite more code that just your program's initialization.

There's also [`femme`](https://github.com/lrlna/femme/) which is one part pretty printer, one part JSON logger, and one part WASM JavaScript object logger. It's strength is that is indeed pretty! It's not _just_ pretty logger and yet also not _just_ a JSON logger. It's an assortment of things making it broadly focused rather than narrowly focused on JSON log formatting. If you only use one of those things you might be packing more than you need.

If you are migrating from `env_logger`'s environment variable driving configuration options you are a bit out of luck. You will be finding yourself recompiling and rebuilding your application to change log levels.

</p>
</details>
&nbsp;

 <details><summary>So what are the tradeoffs of json_env_logger then?</summary>
<p>

Glad you asked. It depends on `env_logger` which has some opinionated defaults, some of which you might not like. For example, it logs to stderr by default. You might play for team stdout. The good news is that json_env_logger exposes its interfaces for overriding those opinions. 

Some features available in `env_logger` `json_env_logger` doesn't use and those bring in extra transitive dependencies. We're aware. Luckily they are all behind `env_logger` feature flags and `json_env_logger` turns them all off! The only transient dependency is then just `log` which you already have if your doing any sort of logging:)
</p>
</details>
&nbsp;

<details><summary>I have more questions</summary>
<p>

 That's technically not a question but ok. Ask away by [opening a GitHub issue](https://github.com/zdannar/json-env-logger2/issues/new). Thanks!
</p>
</details>
&nbsp;

zdannar 2022
