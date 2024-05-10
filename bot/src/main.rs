mod voicevox;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let voicevox = voicevox::VoicevoxService::new("http://voicevox:50021".to_string());
    let text = "こんにちは、世界！";
    let wav = voicevox.tts(text).await?;
    assert_eq!(&wav[0..4], b"RIFF");

    Ok(())
}
