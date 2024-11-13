use crate::cards::PokerCard;
use crate::llms::{EventLLMRequest, EventLLMResponse};
use bevy::prelude::*;
use bevy_la_mesa::events::{DrawHand, RenderDeck};
use bevy_novel::{
    events::{EventHandleNode, EventStartScenario, EventSwitchNextNode},
    NovelData,
};
use renpy_parser::{
    group_logical_lines,
    lexer::Lexer,
    list_logical_lines,
    parsers::{parse_block, AST},
};

use crate::cards::*;

pub(crate) fn load_scenario(path: String) -> Vec<AST> {
    let lines = list_logical_lines(&path).unwrap();
    let blocks = group_logical_lines(lines).unwrap();

    let l = &mut Lexer::new(blocks.clone(), true);

    let (ast, _) = parse_block(l);
    ast
}

pub(crate) fn start_visual_novel(
    ew_start_scenario: EventWriter<EventStartScenario>,
    mut ew_render_deck: EventWriter<RenderDeck<PokerCard>>,
) {
    let path = "assets/plot/intro.rpy";
    let ast = load_scenario(path.to_string());

    // ew_start_scenario.send(EventStartScenario { ast });
    ew_render_deck.send(RenderDeck::<PokerCard> {
        marker: 1,
        deck: load_poker_deck(),
    });
}

pub(crate) fn handle_llm_response(
    mut novel_data: ResMut<NovelData>,
    mut er_llm_response: EventReader<EventLLMResponse>,
    mut ew_switch_next_node: EventWriter<EventSwitchNextNode>,
) {
    for event in er_llm_response.read() {
        novel_data.push_text_node(None, event.0.clone());
        ew_switch_next_node.send(EventSwitchNextNode {});
    }
}

pub(crate) fn handle_new_node(
    mut novel_data: ResMut<NovelData>,
    mut ew_switch_next_node: EventWriter<EventSwitchNextNode>,
    mut er_handle_node: EventReader<EventHandleNode>,
    mut ew_llm_request: EventWriter<EventLLMRequest>,
    mut ew_draw: EventWriter<DrawHand>,
) {
    for event in er_handle_node.read() {
        if let AST::Return(_, _) = event.ast.clone() {
            ew_draw.send(DrawHand {
                deck_marker: 1,
                num_cards: 5,
                player: 1,
            });

            // when sending llm request indicate user that
            novel_data.push_text_node(
                Some("AI".to_string()),
                "I'm meditating the answer".to_string(),
            );
            ew_switch_next_node.send(EventSwitchNextNode {});

            ew_llm_request.send(EventLLMRequest(
                "Say something funny in one simple sentence.".to_string(),
            ));
        }
    }
}
