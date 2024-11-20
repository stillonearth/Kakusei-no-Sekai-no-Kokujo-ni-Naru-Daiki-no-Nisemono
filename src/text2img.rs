use anyhow::anyhow;
use anyhow::Result;
use bevy::prelude::*;
use bevy_tokio_tasks::TokioTasksRuntime;
use image::{self, DynamicImage};
use std::io::Cursor;
use url::{form_urlencoded, Url};

const API_ENDPOINT: &str = "http://167.88.162.83/api";

pub struct Text2ImagePlugin {}

#[derive(Event)]
pub struct EventText2ImageRequest {
    pub prompt: String,
}

#[derive(Event)]
pub struct EventText2ImageResponse {
    pub image: DynamicImage,
}

impl Plugin for Text2ImagePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EventText2ImageRequest>()
            .add_event::<EventText2ImageResponse>()
            .add_systems(Update, handle_text_2_image_request);
    }
}

fn handle_text_2_image_request(
    mut er_text_2_image_request: EventReader<EventText2ImageRequest>,
    runtime: ResMut<TokioTasksRuntime>,
) {
    for er in er_text_2_image_request.read() {
        let prompt = er.prompt.clone();

        runtime.spawn_background_task(|mut ctx| async move {
            let url = format!("{}/image", API_ENDPOINT);
            let mut url = Url::parse(&url).unwrap();

            let encoded_prompt =
                form_urlencoded::byte_serialize(prompt.as_bytes()).collect::<String>();
            url.query_pairs_mut().append_pair("prompt", &encoded_prompt);

            let image = download_and_load_image(&url.to_string()).await;

            if image.is_ok() {
                ctx.run_on_main_thread(move |ctx| {
                    let event_response = EventText2ImageResponse {
                        image: image.unwrap(),
                    };
                    let world: &mut World = ctx.world;
                    world.send_event(event_response);
                })
                .await;
            }
        });
    }
}

async fn download_and_load_image(url: &str) -> Result<DynamicImage> {
    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let image_bytes = response.bytes().await?;
        let img: DynamicImage = image::load(Cursor::new(image_bytes), image::ImageFormat::Jpeg)?;
        Ok(img)
    } else {
        Err(anyhow!(format!("Failed to download image: {}", response.status())).into())
    }
}
