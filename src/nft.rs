use bevy::prelude::*;

use anyhow::Result;
use bevy_wasm_tasks::*;
use renpy_parser::parsers::{ASTVec, AST};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::API_ENDPOINT;

#[derive(Default)]
pub struct NFTPlugin;

#[derive(Event)]
pub struct EventPersistScenarioRequest {
    pub scenario: Vec<AST>,
}

#[derive(Event)]
pub struct EventPersistScenarioResponse {
    pub nft_id: usize,
}

impl Plugin for NFTPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EventPersistScenarioRequest>()
            .add_event::<EventPersistScenarioResponse>()
            .add_systems(Update, handle_persist_nft_request);
    }
}

fn handle_persist_nft_request(
    mut er_llm_request: EventReader<EventPersistScenarioRequest>,
    tasks: Tasks,
) {
    for er in er_llm_request.read() {
        let scenario_string = format!("{}", ASTVec(&er.scenario));

        // TODO: DEDUP
        #[cfg(not(target_arch = "wasm32"))]
        tasks.spawn_tokio(move |ctx| async move {
            let llm_request = NFTPersistRequest {
                scenario: scenario_string,
            };

            let llm_response = api_persist_story(llm_request).await;

            if llm_response.is_ok() {
                let nft_id = llm_response.unwrap();
                ctx.run_on_main_thread(move |ctx| {
                    let event_response = EventPersistScenarioResponse { nft_id };
                    let world: &mut World = ctx.world;
                    world.send_event(event_response);
                })
                .await;
            } else {
                panic!("error: {}", llm_response.err().unwrap());
            }
        });
        // #[cfg(target_arch = "wasm32")]
        // tasks.spawn_wasm(move |ctx| async move {
        //     let llm_request = LLMRequest {
        //         prompt: prompt.clone(),
        //     };
        //     let llm_response = api_llm_request(llm_request).await;
        //     if llm_response.is_ok() {
        //         let llm_response = llm_response.unwrap();
        //         ctx.run_on_main_thread(move |ctx| {
        //             let event_response = EventLLMResponse {
        //                 response: llm_response.clone(),
        //                 who: who.clone(),
        //                 request_type,
        //             };
        //             let world: &mut World = ctx.world;
        //             world.send_event(event_response);
        //         })
        //         .await;
        //     } else {
        //         panic!("error: {}", llm_response.err().unwrap());
        //     }
        // });
    }
}

#[derive(Serialize)]
struct NFTPersistRequest {
    pub scenario: String,
}

#[derive(Deserialize)]
struct NFTPersistResponse {
    pub nft_id: usize,
}

async fn api_persist_story(prompt: NFTPersistRequest) -> Result<usize> {
    let url = format!("{}/nft/save", API_ENDPOINT);
    let payload_json = serde_json::to_string(&prompt)?;

    println!("trying to persist story to blockchain, {}", payload_json);

    let client = Client::new();
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(payload_json)
        .send()
        .await?;
    let response_text = response.text().await?;
    let response: NFTPersistResponse = serde_json::from_str(&response_text)?;

    Ok(response.nft_id)
}
