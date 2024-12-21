use bevy::prelude::*;

use anyhow::Result;
use bevy_wasm_tasks::*;
use reqwest::Client;
use serde::{Deserialize, Serialize};

const API_ENDPOINT: &str = "http://167.88.162.83/api";

pub struct LLMPlugin {}

#[derive(Serialize)]
struct LLMRequest {
    pub prompt: String,
}

#[derive(Deserialize)]
struct LLMResponse {
    pub response: String,
}

#[derive(Clone, Copy)]
pub enum LLMRequestType {
    Story,
    Text2ImagePrompt,
}

#[derive(Event)]
pub struct EventLLMRequest {
    pub prompt: String,
    pub who: Option<String>,
    pub request_type: LLMRequestType,
}

#[derive(Event)]
pub struct EventLLMResponse {
    pub response: String,
    pub who: Option<String>,
    pub request_type: LLMRequestType,
}

impl Plugin for LLMPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EventLLMRequest>()
            .add_event::<EventLLMResponse>()
            .add_systems(Update, handle_llm_request);
    }
}

fn handle_llm_request(mut er_llm_request: EventReader<EventLLMRequest>, tasks: Tasks) {
    for er in er_llm_request.read() {
        let prompt = er.prompt.clone();
        let who = er.who.clone();
        let request_type = er.request_type;

        // TODO: DEDUP
        #[cfg(not(target_arch = "wasm32"))]
        tasks.spawn_tokio(move |ctx| async move {
            let llm_request = LLMRequest {
                prompt: prompt.clone(),
            };

            let llm_response = api_llm_request(llm_request).await;

            if llm_response.is_ok() {
                let llm_response = llm_response.unwrap();
                ctx.run_on_main_thread(move |ctx| {
                    let event_response = EventLLMResponse {
                        response: llm_response.clone(),
                        who: who.clone(),
                        request_type,
                    };
                    let world: &mut World = ctx.world;
                    world.send_event(event_response);
                })
                .await;
            } else {
                panic!("error: {}", llm_response.err().unwrap());
            }
        });
        #[cfg(target_arch = "wasm32")]
        tasks.spawn_wasm(move |ctx| async move {
            let llm_request = LLMRequest {
                prompt: prompt.clone(),
            };

            let llm_response = api_llm_request(llm_request).await;

            if llm_response.is_ok() {
                let llm_response = llm_response.unwrap();
                ctx.run_on_main_thread(move |ctx| {
                    let event_response = EventLLMResponse {
                        response: llm_response.clone(),
                        who: who.clone(),
                        request_type,
                    };
                    let world: &mut World = ctx.world;
                    world.send_event(event_response);
                })
                .await;
            } else {
                panic!("error: {}", llm_response.err().unwrap());
            }
        });
    }
}

async fn api_llm_request(prompt: LLMRequest) -> Result<String> {
    let url = format!("{}/llm", API_ENDPOINT);
    let payload_json = serde_json::to_string(&prompt)?;

    let client = Client::new();
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(payload_json)
        .send()
        .await?;
    let response_text = response.text().await?;
    let response: LLMResponse = serde_json::from_str(&response_text)?;

    Ok(response.response)
}
