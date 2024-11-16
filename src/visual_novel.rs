use bevy::prelude::*;
use bevy_la_mesa::events::RenderDeck;
use bevy_novel::{
    events::{EventHandleNode, EventStartScenario, EventSwitchNextNode},
    NovelData,
};
use renpy_parser::{parse_scenario, parsers::AST};

use crate::llms::*;
use crate::{cards_game::*, GameState};

pub(crate) fn start_visual_novel(mut ew_start_scenario: EventWriter<EventStartScenario>) {
    let path = "assets/plot/intro.rpy";
    let (ast, _) = parse_scenario(path).unwrap();

    ew_start_scenario.send(EventStartScenario { ast });
}

pub(crate) fn handle_llm_response(
    game_state: Res<GameState>,
    mut novel_data: ResMut<NovelData>,
    mut er_llm_response: EventReader<EventLLMResponse>,
    mut ew_switch_next_node: EventWriter<EventSwitchNextNode>,
) {
    for event in er_llm_response.read() {
        println!("Got Response!");

        novel_data.push_text_node(
            Some(event.who.clone()),
            event.response.clone(),
            game_state.current_vn_node + 1,
        );

        println!(
            "nodes len: {:?}\t{}",
            novel_data.ast, game_state.current_vn_node
        );

        ew_switch_next_node.send(EventSwitchNextNode {});
    }
}

pub(crate) fn handle_new_node(
    mut novel_data: ResMut<NovelData>,
    mut game_state: ResMut<GameState>,
    mut ew_switch_next_node: EventWriter<EventSwitchNextNode>,
    mut er_handle_node: EventReader<EventHandleNode>,
    mut ew_llm_request: EventWriter<EventLLMRequest>,
    // mut ew_draw: EventWriter<DrawHand>,
    mut ew_render_deck: EventWriter<RenderDeck<PokerCard>>,
) {
    for event in er_handle_node.read() {
        game_state.current_vn_node = event.ast.index();

        if let AST::LLMGenerate(_, who, prompt) = event.ast.clone() {
            // when sending llm request indicate user that

            novel_data.push_text_node(
                Some("AI".to_string()),
                "...".to_string(),
                game_state.current_vn_node + 1,
            );
            ew_switch_next_node.send(EventSwitchNextNode {});

            println!(
                "nodes len: {:?}\t{}",
                novel_data.ast, game_state.current_vn_node
            );

            let prompt = prompt
                .unwrap()
                .replace(
                    "{COMBINATIONS}",
                    &game_state
                        .combinations
                        .iter()
                        .map(|c| c.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                )
                .replace("{SCORE}", &game_state.score.to_string());

            ew_llm_request.send(EventLLMRequest {
                prompt: prompt,
                who: who,
            });
        }

        if let AST::GameMechanic(_, mechanic) = event.ast.clone() {
            if mechanic == "card play" {
                ew_render_deck.send(RenderDeck::<PokerCard> {
                    marker: 1,
                    deck: load_poker_deck(),
                });
            }
        }
    }
}
