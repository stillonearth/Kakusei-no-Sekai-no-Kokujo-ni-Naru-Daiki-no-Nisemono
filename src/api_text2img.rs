use anyhow::anyhow;
use anyhow::Result;
use bevy::prelude::*;
use bevy_wasm_tasks::*;
use image::{self, DynamicImage};
use reqwest::Client;
use serde::Deserialize;
use std::io::Cursor;
use url::{form_urlencoded, Url};

use crate::API_ENDPOINT;

#[derive(Default)]
pub struct Text2ImagePlugin;

#[derive(Event)]
pub struct EventText2ImageRequest {
    pub prompt: String,
}

#[derive(Event)]
pub struct EventDownloadImageRequest {
    pub filename: String,
}

#[derive(Event)]
pub struct EventDownloadImageResponse {
    pub image: DynamicImage,
    pub filename: String,
}

#[derive(Event)]
pub struct EventText2ImageResponse {
    pub image: DynamicImage,
    pub filename: String,
}

#[derive(Deserialize)]
struct ImageGenerateResponse {
    pub hash: String,
}

impl Plugin for Text2ImagePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EventText2ImageRequest>()
            .add_event::<EventText2ImageResponse>()
            .add_event::<EventDownloadImageRequest>()
            .add_event::<EventDownloadImageResponse>()
            .add_systems(Update, (handle_text_2_image_request, handle_download_image));
    }
}

fn handle_download_image(
    mut er_download_image_request: EventReader<EventDownloadImageRequest>,
    tasks: Tasks,
) {
    for er in er_download_image_request.read() {
        let url = format!(
            "https://kakuseinosekainokokujoninarudaikinonisemono.space/api/image/v2/{}",
            er.filename
        );
        let filename = er.filename.clone();

        // TODO: DEDUP
        #[cfg(not(target_arch = "wasm32"))]
        tasks.spawn_tokio(|ctx| async move {
            let image = download_and_load_image(url.as_ref()).await;

            if image.is_ok() {
                ctx.run_on_main_thread(move |ctx| {
                    let world: &mut World = ctx.world;

                    world.send_event(EventDownloadImageResponse {
                        image: image.unwrap(),
                        filename: filename.clone(),
                    });
                })
                .await;
            }
        });
        #[cfg(target_arch = "wasm32")]
        tasks.spawn_wasm(|ctx| async move {
            let image = download_and_load_image(url.as_ref()).await;

            if image.is_ok() {
                ctx.run_on_main_thread(move |ctx| {
                    let event_response = EventDownloadImageResponse {
                        image: image.unwrap(),
                        filename,
                    };
                    let world: &mut World = ctx.world;
                    world.write_event(event_response);
                })
                .await;
            }
        });
    }
}

fn handle_text_2_image_request(
    mut er_text_2_image_request: EventReader<EventText2ImageRequest>,
    tasks: Tasks,
) {
    for er in er_text_2_image_request.read() {
        let prompt = er.prompt.clone();

        // TODO: DEDUP
        #[cfg(not(target_arch = "wasm32"))]
        tasks.spawn_tokio(|ctx| async move {
            let url = format!("{}/image/v2", API_ENDPOINT);
            let mut url = Url::parse(&url).unwrap();

            let encoded_prompt =
                form_urlencoded::byte_serialize(prompt.as_bytes()).collect::<String>();
            url.query_pairs_mut().append_pair("prompt", &encoded_prompt);

            let image_hash = generate_image(url.as_ref()).await;
            if image_hash.is_err() {
                return;
            }
            let filename = image_hash.unwrap();
            let url = format!("{}/image/v2/{}", API_ENDPOINT, filename);
            let image = download_and_load_image(url.as_ref()).await;

            if image.is_ok() {
                ctx.run_on_main_thread(move |ctx| {
                    let event_response = EventText2ImageResponse {
                        image: image.unwrap(),
                        filename,
                    };
                    let world: &mut World = ctx.world;
                    world.send_event(event_response);
                })
                .await;
            }
        });
        #[cfg(target_arch = "wasm32")]
        tasks.spawn_wasm(|ctx| async move {
            let url = format!("{}/image/v2", API_ENDPOINT);
            let mut url = Url::parse(&url).unwrap();

            let encoded_prompt =
                form_urlencoded::byte_serialize(prompt.as_bytes()).collect::<String>();
            url.query_pairs_mut().append_pair("prompt", &encoded_prompt);

            let image_hash = generate_image(url.as_ref()).await;
            if image_hash.is_err() {
                return;
            }
            let filename = image_hash.unwrap();
            let url = format!("{}/image/v2/{}", API_ENDPOINT, filename);
            let image = download_and_load_image(url.as_ref()).await;

            if image.is_ok() {
                ctx.run_on_main_thread(move |ctx| {
                    let event_response = EventText2ImageResponse {
                        image: image.unwrap(),
                        filename,
                    };
                    let world: &mut World = ctx.world;
                    world.write_event(event_response);
                })
                .await;
            }
        });
    }
}

async fn generate_image(url: &str) -> Result<String> {
    let client = Client::new();
    let response = client.get(url).send().await?;
    let response_text = response.text().await?;
    let response: ImageGenerateResponse = serde_json::from_str(&response_text)?;

    Ok(response.hash)
}

async fn download_and_load_image(url: &str) -> Result<DynamicImage> {
    let response = reqwest::get(url).await?;

    if response.status().is_success() {
        let image_bytes = response.bytes().await?;
        let img: DynamicImage = image::load(Cursor::new(image_bytes), image::ImageFormat::Jpeg)?;
        Ok(img)
    } else {
        Err(anyhow!(format!(
            "Failed to download image: {}",
            response.status()
        )))
    }
}
