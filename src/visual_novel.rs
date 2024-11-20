use bevy::prelude::*;
use bevy_la_mesa::events::RenderDeck;
use bevy_novel::{
    events::{EventHandleNode, EventStartScenario, EventSwitchNextNode},
    NovelData, NovelSettings,
};
use renpy_parser::{parse_scenario, parsers::AST};
use uuid::Uuid;

use crate::{
    cards_game::*,
    llm::*,
    text2img::{EventText2ImageRequest, EventText2ImageResponse},
    EventStartNarrativeGame, EventStartPokerGame, GameState, GameType,
};

const PROMPT: &str = "You are narrator in a visual novel. Create a script for visual novel based on this setting. Respond only with story sentences. Do not include any instructions or explanations. Respond with at least 20 sentences each separated with new line. Each sentence no longer 10 words.";

pub(crate) fn start_visual_novel(mut ew_start_scenario: EventWriter<EventStartScenario>) {
    let path = "assets/plot/intro.rpy";
    let result = parse_scenario(path);

    if result.is_err() {
        panic!("{:?}", result.err());
        return;
    }

    let (ast, _) = result.unwrap();
    ew_start_scenario.send(EventStartScenario { ast });
}

pub(crate) fn handle_text_2_image_response(
    novel_settings: Res<NovelSettings>,
    game_state: Res<GameState>,
    mut novel_data: ResMut<NovelData>,
    mut ew_switch_next_node: EventWriter<EventSwitchNextNode>,
    mut er_text_2_image_response: EventReader<EventText2ImageResponse>,
) {
    for event in er_text_2_image_response.read() {
        let image_name = Uuid::new_v4().to_string();

        event
            .image
            .save(format!(
                "assets/{}/{}.png",
                novel_settings.assets_path, image_name
            ))
            .unwrap();

        novel_data.push_scene_node(image_name, game_state.n_vn_node_scene_request + 1);
        ew_switch_next_node.send(EventSwitchNextNode {});
    }
}

pub(crate) fn handle_llm_response(
    mut game_state: ResMut<GameState>,
    mut novel_data: ResMut<NovelData>,
    mut er_llm_response: EventReader<EventLLMResponse>,
    mut ew_llm_request: EventWriter<EventLLMRequest>,
    // mut ew_switch_next_node: EventWriter<EventSwitchNextNode>,
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
                }

                let text_2_image_prompt = format!(
                    "Create prompt for text-to-image model based short story. Respond only with one prompt. Do not include any explanations. Story:`{}`",
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
    mut ew_switch_next_node: EventWriter<EventSwitchNextNode>,
    mut er_handle_node: EventReader<EventHandleNode>,
    mut ew_llm_request: EventWriter<EventLLMRequest>,
    // mut ew_draw: EventWriter<DrawHand>,
    mut ew_start_poker_game: EventWriter<EventStartPokerGame>,
    mut ew_start_narrative_game: EventWriter<EventStartNarrativeGame>,
) {
    for event in er_handle_node.read() {
        game_state.n_vn_node = event.ast.index();

        if let AST::LLMGenerate(_, who, prompt) = event.ast.clone() {
            // when sending llm request indicate user that

            novel_data.push_text_node(
                Some("AI".to_string()),
                "...".to_string(),
                game_state.n_vn_node + 1,
            );
            ew_switch_next_node.send(EventSwitchNextNode {});

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
                .replace("{SCORE}", &game_state.poker_score.to_string())
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
        }

        if let AST::GameMechanic(_, mechanic) = event.ast.clone() {
            if mechanic == "card play poker" {
                ew_start_poker_game.send(EventStartPokerGame {});
            }

            if mechanic == "card play narrative setting" {
                ew_start_narrative_game.send(EventStartNarrativeGame::Setting);
            }

            if mechanic == "card play narrative conflict" {
                ew_start_narrative_game.send(EventStartNarrativeGame::Conflict);
            }

            if mechanic == "card play narrative plot twist" {
                ew_start_narrative_game.send(EventStartNarrativeGame::PlotTwist);
            }
        }
    }
}

pub(crate) fn handle_start_poker_game(
    mut game_state: ResMut<GameState>,
    mut er_start_poker_game: EventReader<EventStartPokerGame>,
    mut ew_render_deck: EventWriter<RenderDeck<VNCard>>,
) {
    for _ in er_start_poker_game.read() {
        game_state.game_type = GameType::Poker;

        ew_render_deck.send(RenderDeck::<VNCard> {
            marker: 1,
            deck: load_poker_deck(),
        });
    }
}

pub(crate) fn handle_start_narrative_game(
    mut game_state: ResMut<GameState>,
    mut er_start_narrative_game: EventReader<EventStartNarrativeGame>,
    mut ew_render_deck: EventWriter<RenderDeck<VNCard>>,
) {
    for event in er_start_narrative_game.read() {
        game_state.game_type = GameType::Narrative;

        match event {
            EventStartNarrativeGame::Setting => {
                ew_render_deck.send(RenderDeck::<VNCard> {
                    marker: 1,
                    deck: load_narrative_setting_deck().unwrap(),
                });
            }
            EventStartNarrativeGame::PlotTwist => {
                ew_render_deck.send(RenderDeck::<VNCard> {
                    marker: 1,
                    deck: load_narrative_plot_twist_deck().unwrap(),
                });
            }
            EventStartNarrativeGame::Conflict => {
                ew_render_deck.send(RenderDeck::<VNCard> {
                    marker: 1,
                    deck: load_narrative_conflict_deck().unwrap(),
                });
            }
        }
    }
}
