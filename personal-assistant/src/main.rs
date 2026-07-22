use behavioral_memory::BehavioralMemoryPlugin;
use clock::ClockPlugin;
use datadir_local::DataLocalDir;
use documents::DocumentsPlugin;
use episodic_memory::EpisodicMemoryPlugin;
use host_audio::{HostAudioInputPlugin, HostAudioOutputPlugin};
use prompt_file::FilePromptProvider;
use semantic_memory::SemanticMemoryPlugin;
use speaker_recognizer::SpeakerRecognizerPlugin;
use std::process::ExitCode;
use stt_google::SttGooglePlugin;
use synapto::config::{ConfigJson, DotEnv, Env};
use synapto::Synapto;
use synapto_test::local_storage::LocalStorage;
use tts_google::TtsGooglePlugin;

type DataDir = DataLocalDir<"personal-assistant">;
type Storage = LocalStorage<DataDir>;

#[tokio::main]
async fn main() -> ExitCode {
    Synapto::<
        (ConfigJson<DataDir>, DotEnv, Env),
        Storage,
        FilePromptProvider<DataDir>,
    >::run::<(
        EpisodicMemoryPlugin<Storage>,
        ClockPlugin,
        SemanticMemoryPlugin<Storage>,
        DocumentsPlugin<Storage>,
        BehavioralMemoryPlugin<Storage>,
        HostAudioInputPlugin,
        HostAudioOutputPlugin,
        SttGooglePlugin,
        TtsGooglePlugin,
        SpeakerRecognizerPlugin,
    )>()
    .await
}
