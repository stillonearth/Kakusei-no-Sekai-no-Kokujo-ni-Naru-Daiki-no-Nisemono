use bevy::prelude::*;
use bevy_defer::AsyncCommandsExtension;
use bevy_defer::AsyncWorld;
use bevy_la_mesa::{
    events::{DrawToHand, PlaceCardOnTable},
    Card, Hand, PlayArea,
};

use crate::{cards_game::VNCard, EventUpdateGameStateUI, GameState};

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
    mut ew_update_game_state_ui: EventWriter<EventUpdateGameStateUI>,
) {
    for event in card_position_press.read() {
        if q_cards_in_hand.iter().len() == 0 {
            return;
        }

        let (card_entity, _, _) = q_cards_in_hand.single();

        if let Ok((_, mut visibility, area)) = q_play_areas.get_mut(event.entity) {
            ew_place_card_on_table.send(PlaceCardOnTable {
                card_entity,
                player: 1,
                marker: area.marker,
            });

            if game_state.n_draws < game_state.max_n_poker_draws {
                commands.spawn_task(|| async move {
                    AsyncWorld.sleep(0.5).await;
                    AsyncWorld.send_event(DrawToHand {
                        deck_marker: 1,
                        num_cards: 1,
                        player: 1,
                    })?;
                    Ok(())
                });
            } else {
                game_state.ui_enable_play_hand = true;
                ew_update_game_state_ui.send(EventUpdateGameStateUI {});
            }

            *visibility = Visibility::Hidden;
        }
    }
}
