use bevy::prelude::*;
use bevy_defer::AsyncCommandsExtension;
use bevy_defer::AsyncWorld;
use bevy_la_mesa::DeckArea;
use bevy_la_mesa::{
    events::{DrawToHand, PlaceCardOnTable},
    Card, Hand, PlayArea,
};

use crate::game_menu::EventRefreshUI;
use crate::game_menu::PokerMenuSettings;
use crate::{cards_game::VNCard, GameState};

#[derive(Event)]
pub struct EventCardPositionHover {
    pub entity: Entity,
}

#[derive(Event)]
pub struct EventCardPositionOut {
    pub entity: Entity,
}

#[derive(Event)]
pub struct EventCardPositionPress {
    pub entity: Entity,
}

// Event Handlers
pub fn handle_card_position_hover(
    mut hover: EventReader<EventCardPositionHover>,
    mut query: Query<(Entity, &mut MeshMaterial3d<StandardMaterial>, &PlayArea)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    hover.read().for_each(|hover| {
        if let Ok((_, mut material, _)) = query.get_mut(hover.entity) {
            *material = MeshMaterial3d(materials.add(Color::srgb_u8(255, 144, 255)));
        }
    });
}

pub fn handle_card_position_out(
    mut hover: EventReader<EventCardPositionOut>,
    mut query: Query<(Entity, &mut MeshMaterial3d<StandardMaterial>, &PlayArea)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    hover.read().for_each(|hover| {
        if let Ok((_, mut material, _)) = query.get_mut(hover.entity) {
            *material = MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255)));
        }
    });
}

pub fn handle_card_position_press(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut card_position_press: EventReader<EventCardPositionPress>,
    mut ew_place_card_on_table: EventWriter<PlaceCardOnTable>,
    q_cards_in_hand: Query<(Entity, &Card<VNCard>, &Hand)>,
    mut q_play_areas: Query<(Entity, &mut Visibility, &PlayArea)>,
    q_decks: Query<(Entity, &DeckArea)>,
    mut ew_refresh_ui: EventWriter<EventRefreshUI>,
) {
    for event in card_position_press.read() {
        if q_cards_in_hand.iter().len() == 0 {
            return;
        }

        let (card_entity, _, _) = q_cards_in_hand.single();
        let main_deck_entity = q_decks.iter().find(|(_, deck)| deck.marker == 1).unwrap().0;

        if let Ok((_, mut visibility, area)) = q_play_areas.get_mut(event.entity) {
            if *visibility == Visibility::Hidden {
                continue;
            }

            ew_place_card_on_table.send(PlaceCardOnTable {
                card_entity,
                player: 1,
                marker: area.marker,
            });
            game_state.n_turns += 1;

            if game_state.n_draws < game_state.max_n_poker_draws {
                commands.spawn_task(move || async move {
                    AsyncWorld.sleep(0.5).await;
                    AsyncWorld.send_event(DrawToHand {
                        deck_entity: main_deck_entity,
                        num_cards: 1,
                        player: 1,
                    })?;
                    Ok(())
                });
                ew_refresh_ui.send(EventRefreshUI::PokerMenu(PokerMenuSettings {
                    show_advance_button: false,
                    show_score: false,
                    score: game_state.score as usize,
                }));
            } else {
                ew_refresh_ui.send(EventRefreshUI::PokerMenu(PokerMenuSettings {
                    show_advance_button: true,
                    show_score: false,
                    score: game_state.score as usize,
                }));
            }

            *visibility = Visibility::Hidden;
        }
    }
}
