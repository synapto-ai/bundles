use clock::ClockPlugin;
use datadir_local::DataLocalDir;
use prompt_file::FilePromptProvider;
use std::process::ExitCode;
use stt_google::SttGooglePlugin;
use synapto::Synapto;
use synapto::config::{ConfigJson, DotEnv, Env};
use synapto_test::local_storage::LocalStorage;
use tts_google::TtsGooglePlugin;

type DataDir = DataLocalDir<"teacher">;
type Storage = LocalStorage<DataDir>;

#[tokio::main]
async fn main() -> ExitCode {
    Synapto::<(ConfigJson<DataDir>, DotEnv, Env), Storage, FilePromptProvider<DataDir>>::run::<(
        ClockPlugin,
        SttGooglePlugin,
        TtsGooglePlugin,
    )>()
    .await
}
