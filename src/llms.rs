use bevy::ecs::world::CommandQueue;
use bevy::{prelude::*, tasks::AsyncComputeTaskPool};

use bevy_tokio_tasks::TokioTasksRuntime;
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;

pub struct LLMPlugin {}

#[derive(Event)]
pub struct EventLLMRequest(pub String);

#[derive(Event)]
pub struct EventLLMResponse(pub String);

impl Plugin for LLMPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EventLLMRequest>()
            .add_event::<EventLLMResponse>()
            .add_systems(Update, handle_llm_request);
    }
}

fn handle_llm_request(
    mut er_llm_request: EventReader<EventLLMRequest>,
    runtime: ResMut<TokioTasksRuntime>,
) {
    for er in er_llm_request.read() {
        let prompt = er.0.clone();
        runtime.spawn_background_task(|mut ctx| async move {
            let model = "llama3.2:3b".to_string();
            let ollama = Ollama::new("http://192.168.88.242".to_string(), 11434);

            println!("requesting llm");

            let res = ollama.generate(GenerationRequest::new(model, prompt)).await;

            if res.is_ok() {
                let response = res.unwrap().response;

                println!("success: {}", response);

                ctx.run_on_main_thread(move |ctx| {
                    let event_response = EventLLMResponse(response);
                    let world: &mut World = ctx.world;
                    world.send_event(event_response);
                })
                .await;
            } else {
                println!("error");
            }
        });
    }
}
