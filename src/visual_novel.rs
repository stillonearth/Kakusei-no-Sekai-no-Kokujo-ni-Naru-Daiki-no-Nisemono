use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_novel::{
    events::{EventHandleNode, EventHideTextNode, EventStartScenario, EventSwitchNextNode},
    rpy_asset_loader::Rpy,
    NovelData, NovelSettings,
};
use image::DynamicImage;
use renpy_parser::parsers::AST;
use uuid::Uuid;

use crate::{
    cards_game::{filter_initial_narrative_cards, NarrativeCards, VNCard, VNCardMetadata},
    llm::*,
    text2img::{EventText2ImageRequest, EventText2ImageResponse},
    AppState, EventStartNarrativeCardShop, EventStartNarrativeGame, EventStartPokerGame, GameState,
    NarrativeCardsHandle, ScenarioHandle,
};

const PROMPT: &'static str = r#"
You are narrator in a visual novel.
Create a script for visual novel based on this setting.
Respond only with story sentences.
Do not include any instructions or explanations.
Respond with at least 20 sentences each separated with new line. Each sentence no longer 10 words.
"#;

pub fn start_visual_novel(
    mut ew_start_scenario: EventWriter<EventStartScenario>,
    scenario_handle: Res<ScenarioHandle>,
    rpy_assets: Res<Assets<Rpy>>,
    narrative_cards_handle: Res<NarrativeCardsHandle>,
    narrative_cards_assets: Res<Assets<NarrativeCards>>,
    mut app_state: ResMut<NextState<AppState>>,
    mut game_state: ResMut<GameState>,
) {
    if let Some(rpy) = rpy_assets.get(scenario_handle.id())
        && let Some(narrative_cards) = narrative_cards_assets.get(narrative_cards_handle.id())
    {
        ew_start_scenario.send(EventStartScenario { ast: rpy.0.clone() });

        let mut deck: Vec<VNCard> = vec![];
        for (i, narrative_card) in narrative_cards.iter().enumerate() {
            if i > 63 {
                break;
            }
            deck.push(VNCard {
                filename: format!("narrative-cards/card-{}.png", i + 1),
                metadata: VNCardMetadata::Narrative(
                    i + 1,
                    narrative_card.card_type.clone(),
                    narrative_card.genre.clone(),
                    narrative_card.name.clone(),
                    narrative_card.effect.clone(),
                    narrative_card.price,
                ),
            });
        }

        game_state.game_deck = deck.clone();
        game_state.collected_deck = filter_initial_narrative_cards(deck.clone());
        app_state.set(AppState::Novel);
    }
}

pub(crate) fn handle_text_2_image_response(
    game_state: ResMut<GameState>,
    mut novel_data: ResMut<NovelData>,
    mut ew_switch_next_node: EventWriter<EventSwitchNextNode>,
    mut er_text_2_image_response: EventReader<EventText2ImageResponse>,
    mut textures: ResMut<Assets<Image>>,
) {
    for event in er_text_2_image_response.read() {
        let image_name = Uuid::new_v4().to_string();
        let dynamic_image: DynamicImage = event.image.clone();
        let rgba_image = dynamic_image.to_rgba8();
        let texture = Image::new_fill(
            Extent3d {
                width: rgba_image.width(),
                height: rgba_image.height(),
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            &rgba_image,
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::all(),
        );

        let texture_handle = textures.add(texture);
        let sprite = Sprite {
            image: texture_handle.clone(),
            ..Default::default()
        };

        novel_data.write_image_cache(image_name.clone(), sprite);
        novel_data.push_scene_node(image_name.clone(), game_state.n_vn_node_scene_request + 1);
        ew_switch_next_node.send(EventSwitchNextNode {});
    }
}

pub(crate) fn handle_llm_response(
    mut game_state: ResMut<GameState>,
    mut novel_settings: ResMut<NovelSettings>,
    mut novel_data: ResMut<NovelData>,
    mut er_llm_response: EventReader<EventLLMResponse>,
    mut ew_llm_request: EventWriter<EventLLMRequest>,
    mut ew_text_2_image_reqeust: EventWriter<EventText2ImageRequest>,
) {
    for event in er_llm_response.read() {
        match event.request_type {
            LLMRequestType::Story => {
                let sentences = event
                    .response
                    .split("\n")
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<_>>();

                for (i, sentence) in sentences.iter().enumerate() {
                    let sentence = sentence.to_string();
                    game_state.narrative_story_so_far.push(sentence.clone());

                    novel_data.push_text_node(
                        event.who.clone(),
                        sentence.clone(),
                        game_state.n_vn_node + 1 + i,
                    );

                    novel_settings.pause_handle_switch_node = false;
                }

                let text_2_image_prompt = format!(
                    r#"
                    Create prompt for text-to-image model based short story. 
                    Respond only with one prompt. 
                    Do not include any explanations. 
                    Story:`{}`
                    "#,
                    sentences.join(" ")
                );

                ew_llm_request.send(EventLLMRequest {
                    prompt: text_2_image_prompt,
                    who: None,
                    request_type: LLMRequestType::Text2ImagePrompt,
                });

                game_state.n_vn_node_scene_request = game_state.n_vn_node;
            }
            LLMRequestType::Text2ImagePrompt => {
                ew_text_2_image_reqeust.send(EventText2ImageRequest {
                    prompt: event.response.clone(),
                });
                // ew_switch_next_node.send(EventSwitchNextNode {});
            }
        }
    }
}

pub(crate) fn handle_new_vn_node(
    mut novel_data: ResMut<NovelData>,
    mut game_state: ResMut<GameState>,
    mut novel_settings: ResMut<NovelSettings>,
    mut ew_switch_next_node: EventWriter<EventSwitchNextNode>,
    mut er_handle_node: EventReader<EventHandleNode>,
    mut ew_llm_request: EventWriter<EventLLMRequest>,
    mut ew_start_poker_game: EventWriter<EventStartPokerGame>,
    mut ew_start_narrative_game: EventWriter<EventStartNarrativeGame>,
    mut ew_start_narrative_card_shop: EventWriter<EventStartNarrativeCardShop>,
    mut ew_hide_vn_text_node: EventWriter<EventHideTextNode>,
) {
    for event in er_handle_node.read() {
        game_state.n_vn_node = event.ast.index();

        if let AST::LLMGenerate(_, who, prompt) = event.ast.clone() {
            novel_data.push_text_node(
                Some("".to_string()),
                "...".to_string(),
                game_state.n_vn_node + 1,
            );
            ew_switch_next_node.send(EventSwitchNextNode {});
            novel_settings.pause_handle_switch_node = true;

            let prompt = prompt
                .unwrap()
                .replace(
                    "{COMBINATIONS}",
                    &game_state
                        .poker_combinations
                        .iter()
                        .map(|c| c.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                )
                .replace("{SCORE}", &game_state.score.to_string())
                .replace("{SETTING}", &game_state.narrative_settings.join(" "))
                .replace("{PLOT TWIST}", &game_state.narrative_plot_twists.join(" "))
                .replace("{CONFLICT}", &game_state.narrative_conflicts.join(" "))
                .replace("{STORY}", &game_state.narrative_story_so_far.join(" "))
                .replace("{PROMPT}", PROMPT);

            ew_llm_request.send(EventLLMRequest {
                prompt,
                who: Some(who),
                request_type: LLMRequestType::Story,
            });
        } else {
            novel_settings.pause_handle_switch_node = false;
        }

        if let AST::GameMechanic(_, mechanic) = event.ast.clone() {
            ew_hide_vn_text_node.send(EventHideTextNode {});
            novel_settings.pause_handle_switch_node = true;

            match mechanic.as_str() {
                "card play poker" => {
                    ew_start_poker_game.send(EventStartPokerGame {});
                }
                "card play narrative setting" => {
                    ew_start_narrative_game.send(EventStartNarrativeGame::Setting);
                }
                "card play narrative conflict" => {
                    ew_start_narrative_game.send(EventStartNarrativeGame::Conflict);
                }
                "card play narrative plot twist" => {
                    ew_start_narrative_game.send(EventStartNarrativeGame::PlotTwist);
                }
                "card shop" => {
                    ew_start_narrative_card_shop.send(EventStartNarrativeCardShop {});
                }
                _ => (), // Handle unexpected mechanic if needed
            }
        }
    }
}
