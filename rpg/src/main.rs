use clock::ClockPlugin;
use datadir_local::DataLocalDir;
use host_audio::{HostAudioInputPlugin, HostAudioOutputPlugin};
use prompt_file::FilePromptProvider;
use speaker_recognizer::SpeakerRecognizerPlugin;
use std::process::ExitCode;
use stt_speechmatics::SttSpeechmaticsPlugin;
use synapto::config::{ConfigJson, DotEnv, Env};
use synapto::Synapto;
use synapto_test::local_storage::LocalStorage;
use tts_google::TtsGooglePlugin;

type DataDir = DataLocalDir<"rpg">;
type Storage = LocalStorage<DataDir>;

#[tokio::main]
async fn main() -> ExitCode {
    Synapto::<
        (ConfigJson<DataDir>, DotEnv, Env),
        Storage,
        FilePromptProvider<DataDir>,
    >::run::<(
        ClockPlugin,
        HostAudioInputPlugin,
        HostAudioOutputPlugin,
        SttSpeechmaticsPlugin,
        TtsGooglePlugin,
        SpeakerRecognizerPlugin,
    )>()
    .await
}
