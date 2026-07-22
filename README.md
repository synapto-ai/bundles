# Creating a Custom Bundle

A bundle is simply a lightweight Rust binary that boots the Synapto core and registers the exact plugins you want.

## How to Create a Custom Bundle

1. Create a new Rust binary crate (e.g. `cargo new my-assistant`).
2. Add `synapto` and the plugins you want to use as dependencies in your `Cargo.toml`.
3. In your `main.rs`, initialize the core with your chosen configuration provider, storage, prompt provider, and plugins. Keep types short by defining type aliases for your data directory and storage strategy.

Here is what a complete, working custom bundle looks like:

```rust
use datadir_local::DataLocalDir;
use host_audio::{HostAudioInputPlugin, HostAudioOutputPlugin};
use prompt_file::FilePromptProvider;
use std::process::ExitCode;
use synapto::config::{ConfigJson, DotEnv, Env};
use synapto::Synapto;
use synapto_test::local_storage::LocalStorage;

type DataDir = DataLocalDir<"my-custom-assistant">;
type Storage = LocalStorage<DataDir>;

#[tokio::main]
async fn main() -> ExitCode {
    // 1. Initialize the core with configuration providers, storage, and prompt provider
    Synapto::<
        (ConfigJson<DataDir>, DotEnv, Env),
        Storage,
        FilePromptProvider<DataDir>,
    >::run::<(
        HostAudioInputPlugin,
        HostAudioOutputPlugin,
        // MyCustomPlugin
    )>()
    // 2. Let it run forever!
    .await
}
```
