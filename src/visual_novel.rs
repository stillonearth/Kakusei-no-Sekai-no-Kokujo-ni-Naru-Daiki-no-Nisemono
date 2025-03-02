use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use bevy_novel::{
    events::{
        EventHandleNode, EventHideTextNode, EventShowTextNode, EventStartScenario,
        EventSwitchNextNode,
    },
    rpy_asset_loader::Rpy,
    NovelData, NovelSettings, NovelText,
};
use image::DynamicImage;
use renpy_parser::parsers::AST;
use uuid::Uuid;

use crate::{
    cards_game::{
        filter_character_deck, filter_initial_character_cards, filter_initial_narrative_cards,
    },
    game_menu::EventRefreshUI,
    llm::*,
    text2img::{EventText2ImageRequest, EventText2ImageResponse},
    EventStartNarrativeCardShop, EventStartNarrativeGame, EventStartPokerGame, GameState, GameType,
    ScenarioHandle,
};

const PROMPT: &str = r#"
You are narrator in a visual novel.
Create a script for visual novel based on this setting.
Respond only with story sentences and character lines.
Do not include any instructions or explanations.
Include dialogues for provived characters in format: "Character -> line". If character description is not provided make dialogue third person.
Respond with at least 20 sentences each separated with new line. Each sentence no longer 10 words.
"#;

pub fn start_visual_novel(
    mut ew_start_scenario: EventWriter<EventStartScenario>,
    scenario_handle: Res<ScenarioHandle>,
    rpy_assets: Res<Assets<Rpy>>,
    mut game_state: ResMut<GameState>,
    mut q_novel_text: Query<(Entity, &mut Node, &NovelText)>,
) {
    if let Some(rpy) = rpy_assets.get(scenario_handle.id()) {
        ew_start_scenario.send(EventStartScenario { ast: rpy.0.clone() });

        game_state.collected_deck = [
            filter_initial_narrative_cards(game_state.game_deck.clone()),
            filter_initial_character_cards(game_state.game_deck.clone()),
        ]
        .concat();
    }

    for (_, mut node, _) in q_novel_text.iter_mut() {
        node.left = Val::Percent(20.0);
        node.margin = UiRect::new(Val::Px(20.0), Val::Px(0.0), Val::Px(0.0), Val::Px(0.0));
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
    assets: Res<AssetServer>,
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

                let mut ast_position: usize = 0;
                for sentence in sentences.iter() {
                    let sentence = sentence.to_string();
                    game_state.narrative_story_so_far.push(sentence.clone());

                    if sentence.contains("->") {
                        let parts: Vec<_> = sentence.split("->").collect();
                        let who = parts[0];
                        let who = who.trim().to_string();
                        let what = parts[1].replace("\"", "");
                        let what = what.trim().to_string();

                        // find appropriate image
                        let character_cards =
                            filter_character_deck(game_state.game_deck.clone()).unwrap();
                        if let Some(character_card) = character_cards
                            .iter()
                            .find(|card| card.metadata.name().unwrap() == who)
                        {
                            let character_name = character_card.metadata.name().unwrap();
                            let image_path = format!("character-cards/{}.png", character_name);
                            let sprite = Sprite::from_image(assets.load(image_path));

                            novel_data.write_image_cache(character_name.clone(), sprite);

                            novel_data.push_show_node(
                                character_name.clone(),
                                game_state.n_vn_node + 1 + ast_position,
                            );
                            ast_position += 1;

                            novel_data.push_text_node(
                                Some(who),
                                what,
                                game_state.n_vn_node + 1 + ast_position,
                            );
                            ast_position += 1;

                            novel_data.push_hide_node(
                                character_name.clone(),
                                game_state.n_vn_node + 1 + ast_position,
                            );
                            ast_position += 1;
                        } else {
                            novel_data.push_text_node(
                                Some(who),
                                what,
                                game_state.n_vn_node + 1 + ast_position,
                            );

                            ast_position += 1;
                        }
                    } else {
                        novel_data.push_text_node(
                            None,
                            sentence.clone(),
                            game_state.n_vn_node + 1 + ast_position,
                        );

                        ast_position += 1;
                    }

                    novel_settings.pause_handle_switch_node = false;
                }

                let text_2_image_prompt = format!(
                    r#"
                    Create prompt for text-to-image model based short story.
                    Image style: realistic.
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
    mut ew_show_vn_text_node: EventWriter<EventShowTextNode>,
    mut ew_refresh_ui: EventWriter<EventRefreshUI>,
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
                .replace("{CHARACTERS}", &game_state.characters.join(" "))
                .replace("{PROMPT}", PROMPT);

            ew_llm_request.send(EventLLMRequest {
                prompt,
                who: Some(who),
                request_type: LLMRequestType::Story,
            });
            ew_refresh_ui.send(EventRefreshUI::LoadingMenu);
            game_state.game_type = GameType::VisualNovel;
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
                "card play narrative characters" => {
                    ew_start_narrative_game.send(EventStartNarrativeGame::Characters);
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
                _ => (),
            }
        } else {
            ew_show_vn_text_node.send(EventShowTextNode {});
            ew_refresh_ui.send(EventRefreshUI::NovelMenu);
            game_state.game_type = GameType::VisualNovel;
        }
    }
}
