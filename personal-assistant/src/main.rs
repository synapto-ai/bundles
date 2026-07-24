use clock::ClockPlugin;
use datadir_local::DataLocalDir;
use host_audio::{HostAudioInputPlugin, HostAudioOutputPlugin};
use mcp::McpPlugin;
use prompt_file::FilePromptProvider;
use std::process::ExitCode;
use stt_google::SttGooglePlugin;
use synapto::Synapto;
use synapto::config::{ConfigJson, DotEnv, Env};
use synapto_interface::data_dir::CurrentDir;
use synapto_test::local_storage::LocalStorage;
use tts_google::TtsGooglePlugin;

type DataDir = DataLocalDir<"personal-assistant">;
type Storage = LocalStorage<DataDir>;

#[tokio::main]
async fn main() -> ExitCode {
    Synapto::<
        (ConfigJson<CurrentDir>, DotEnv, Env),
        Storage,
        FilePromptProvider<CurrentDir>,
    >::run::<(
        ClockPlugin,
        McpPlugin,
        HostAudioInputPlugin,
        HostAudioOutputPlugin,
        SttGooglePlugin,
        TtsGooglePlugin,
    )>()
    .await
}
