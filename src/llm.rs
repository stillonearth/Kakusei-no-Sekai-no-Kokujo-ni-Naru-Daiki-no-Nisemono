use bevy::prelude::*;

use bevy_tokio_tasks::TokioTasksRuntime;
use ollama_rs::generation::completion::request::GenerationRequest;
use ollama_rs::Ollama;

pub struct LLMPlugin {}

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

fn handle_llm_request(
    mut er_llm_request: EventReader<EventLLMRequest>,
    runtime: ResMut<TokioTasksRuntime>,
) {
    for er in er_llm_request.read() {
        let prompt = er.prompt.clone();
        let who = er.who.clone();
        let request_type = er.request_type.clone();

        runtime.spawn_background_task(move |mut ctx| async move {
            let model = "llama3.2:3b".to_string();
            let ollama = Ollama::new("http://192.168.88.242".to_string(), 11434);

            let res = ollama.generate(GenerationRequest::new(model, prompt)).await;

            if res.is_ok() {
                let response = res.unwrap().response;

                ctx.run_on_main_thread(move |ctx| {
                    let event_response = EventLLMResponse {
                        response: response.clone(),
                        who: who.clone(),
                        request_type: request_type.clone(),
                    };
                    let world: &mut World = ctx.world;
                    world.send_event(event_response);
                })
                .await;
            } else {
                panic!("{:?}", res.err());
            }
        });
    }
}
