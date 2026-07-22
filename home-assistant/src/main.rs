use behavioral_memory::BehavioralMemoryPlugin;
use clock::ClockPlugin;
use datadir_local::DataLocalDir;
use mumble::MumblePlugin;
use prompt_file::FilePromptProvider;
use speaker_recognizer::SpeakerRecognizerPlugin;
use std::process::ExitCode;
use stt_google::SttGooglePlugin;
use synapto::config::{ConfigJson, DotEnv, Env};
use synapto::Synapto;
use synapto_test::local_storage::LocalStorage;
use tts_google::TtsGooglePlugin;

type DataDir = DataLocalDir<"home-assistant">;
type Storage = LocalStorage<DataDir>;

#[tokio::main]
async fn main() -> ExitCode {
    Synapto::<
        (ConfigJson<DataDir>, DotEnv, Env),
        Storage,
        FilePromptProvider<DataDir>,
    >::run::<(
        MumblePlugin,
        ClockPlugin,
        BehavioralMemoryPlugin<Storage>,
        SttGooglePlugin,
        TtsGooglePlugin,
        SpeakerRecognizerPlugin,
    )>()
    .await
}
