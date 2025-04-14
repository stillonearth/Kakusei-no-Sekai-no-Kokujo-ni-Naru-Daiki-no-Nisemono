use bevy::prelude::*;

use anyhow::Result;
use bevy_novel::events::EventStartScenario;
use bevy_wasm_tasks::*;
use renpy_parser::{
    parse_scenario_from_string,
    parsers::{ASTVec, AST},
};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{menu_game::EventRefreshUI, GameState, ScenarioHandle, API_ENDPOINT};

#[derive(Default)]
pub struct NFTPlugin;

// Persist NFT

#[derive(Event)]
pub struct EventPersistScenarioRequest {
    pub scenario: Vec<AST>,
}

#[derive(Event)]
pub struct EventPersistScenarioResponse {
    pub nft_id: usize,
}

// Load NFT

#[derive(Event)]
pub struct EventLoadNFTRequest {
    pub url: String,
}

#[derive(Event)]
pub struct EventLoadNFTResponse {
    pub nft: StoryNFT,
}

impl Plugin for NFTPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EventPersistScenarioRequest>()
            .add_event::<EventPersistScenarioResponse>()
            .add_event::<EventLoadNFTRequest>()
            .add_event::<EventLoadNFTResponse>()
            .add_systems(
                Update,
                (
                    handle_persist_nft_request,
                    handle_persist_nft_response,
                    handle_load_nft_request,
                    handle_load_nft_response,
                ),
            );
    }
}

fn handle_persist_nft_response(
    mut er_llm_response: EventReader<EventPersistScenarioResponse>,
    mut ew_refresh_ui: EventWriter<EventRefreshUI>,
) {
    for event in er_llm_response.read() {
        ew_refresh_ui.send(EventRefreshUI::GameOver(event.nft_id));
    }
}

fn handle_persist_nft_request(
    mut er_llm_request: EventReader<EventPersistScenarioRequest>,
    tasks: Tasks,
    game_state: Res<GameState>,
) {
    for er in er_llm_request.read() {
        let scenario_string = format!("{}", ASTVec(&er.scenario));

        // remove all lines starting with "game_mechanic" and "llm_generate"
        let scenario_string = scenario_string
            .lines()
            .filter(|line| {
                let line = line.trim_start();
                !line.starts_with("game_mechanic")
                    && !line.starts_with("llm_generate")
                    && !line.starts_with("#game_mechanic")
            })
            .collect::<Vec<&str>>()
            .join("\n");

        let owner = game_state.wallet.address.clone();

        // TODO: DEDUP
        #[cfg(not(target_arch = "wasm32"))]
        tasks.spawn_tokio(move |ctx| async move {
            let llm_request = NFTPersistRequest {
                scenario: scenario_string,
                owner,
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
        #[cfg(target_arch = "wasm32")]
        tasks.spawn_wasm(move |ctx| async move {
            let llm_request = NFTPersistRequest {
                scenario: scenario_string,
                owner,
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
    }
}

fn handle_load_nft_request(
    mut er_llm_request: EventReader<EventLoadNFTRequest>,
    tasks: Tasks,
    // game_state: Res<GameState>,
) {
    for er in er_llm_request.read() {
        let url = er.url.clone();

        // TODO: DEDUP
        #[cfg(not(target_arch = "wasm32"))]
        tasks.spawn_tokio(move |ctx| async move {
            let nft_response = api_load_nft(url).await;

            if nft_response.is_ok() {
                let nft = nft_response.unwrap();

                ctx.run_on_main_thread(move |ctx| {
                    let event_response = EventLoadNFTResponse { nft };
                    let world: &mut World = ctx.world;
                    world.send_event(event_response);
                })
                .await;
            } else {
                panic!("error: {}", nft_response.err().unwrap());
            }
        });
        #[cfg(target_arch = "wasm32")]
        tasks.spawn_wasm(move |ctx| async move {
            let nft_response = api_load_nft(url).await;

            if nft_response.is_ok() {
                let nft = nft_response.unwrap();

                ctx.run_on_main_thread(move |ctx| {
                    let event_response = EventLoadNFTResponse { nft };
                    let world: &mut World = ctx.world;
                    world.send_event(event_response);
                })
                .await;
            } else {
                panic!("error: {}", nft_response.err().unwrap());
            }
        });
    }
}

fn handle_load_nft_response(
    mut ew_start_scenario: EventWriter<EventStartScenario>,
    mut er_load_nft_response: EventReader<EventLoadNFTResponse>,
    // mut ew_refresh_ui: EventWriter<EventRefreshUI>,
) {
    for event in er_load_nft_response.read() {
        let scenario_string = event.nft.scenario.clone();

        let result = parse_scenario_from_string(&scenario_string, "_");
        if let Ok((scenario, _)) = result {
            ew_start_scenario.send(EventStartScenario {
                ast: scenario.clone(),
            });
        } else {
            println!("could not load scenario {:?}", result);
        }
    }
}

#[derive(Serialize)]
struct NFTPersistRequest {
    pub scenario: String,
    pub owner: String,
}

#[derive(Deserialize, Debug)]
struct NFTPersistResponse {
    pub nft_id: usize,
}

async fn api_persist_story(prompt: NFTPersistRequest) -> Result<usize> {
    let url = format!("{}/nft/create", API_ENDPOINT);
    let payload_json = serde_json::to_string(&prompt)?;

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

#[derive(Deserialize, Debug)]
struct StoryNFT {
    pub description: String,
    pub image: String,
    pub name: String,
    pub poster: String,
    pub scenario: String,
}

async fn api_load_nft(url: String) -> Result<StoryNFT> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    let response_text = response.text().await?;
    let nft: StoryNFT = serde_json::from_str(&response_text)?;

    Ok(nft)
}
