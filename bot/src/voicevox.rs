use anyhow::Result;
use serde::Serialize;

pub struct VoicevoxService {
    http_client: reqwest::Client,
    host: String,
}

impl VoicevoxService {
    pub fn new(host: String) -> Self {
        Self {
            http_client: reqwest::Client::new(),
            host,
        }
    }

    pub async fn tts(&self, text: &str) -> Result<Vec<u8>> {
        let audio_query_input = AudioQueryInput {
            text: text.to_string(),
            speaker: Some("3".to_string()),
            speed: Some(1.3),
        };
        let audio_query = self.get_audio_query(&audio_query_input).await?;

        let wav = reqwest::Client::new()
            .post(&format!("{}/synthesis", self.host))
            .header("Content-Type", "application/json")
            .query(&SynthesisQuery {
                speaker: "3".to_string(),
            })
            .body(audio_query.to_string())
            .send()
            .await?;

        Ok(wav.bytes().await?.to_vec())
    }

    async fn get_audio_query(&self, input: &AudioQueryInput) -> Result<serde_json::Value> {
        let audio_query = self
            .http_client
            .post(&format!("{}/audio_query", self.host))
            .query(&input)
            .send()
            .await?;
        let mut audio_query_json: serde_json::Value =
            serde_json::from_str(&audio_query.text().await?)?;
        audio_query_json["speedScale"] =
            serde_json::Value::String(input.speed.unwrap().to_string());
        Ok(audio_query_json)
    }
}

#[derive(Serialize)]
struct TtsRequest {
    audio_query: serde_json::Value,
}

#[derive(Serialize)]
struct AudioQueryInput {
    text: String,
    speaker: Option<String>,
    speed: Option<f32>,
}

#[derive(Serialize)]
pub struct SynthesisQuery {
    pub speaker: String,
}
