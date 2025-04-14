use bevy::prelude::*;
use bevy_defer::AsyncCommandsExtension;
use bevy_defer::AsyncWorld;
use bevy_la_mesa::CardOnTable;
use bevy_la_mesa::DeckArea;
use bevy_la_mesa::{
    events::{DrawToHand, PlaceCardOnTable},
    Card, Hand, PlayArea,
};

use crate::cards_game::check_poker_hand;
use crate::menu_game::EventRefreshUI;
use crate::menu_game::PokerMenuSettings;
use crate::GameType;
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
            }

            *visibility = Visibility::Hidden;
        }
    }
}

pub fn poker_handle_place_card_on_table(
    mut er_place_card_on_table: EventReader<PlaceCardOnTable>,
    q_cards_on_table: Query<(Entity, &Card<VNCard>, &CardOnTable)>,
    mut ew_refresh_ui: EventWriter<EventRefreshUI>,
    game_state: ResMut<GameState>,
) {
    if game_state.game_type != GameType::Poker {
        return;
    }

    for _ in er_place_card_on_table.read() {
        let poker_cards_on_table = q_cards_on_table
            .iter()
            .map(|(_, card, card_on_table)| {
                (
                    card.data.clone(),
                    card_on_table.marker / 5,
                    card_on_table.marker % 5,
                )
            })
            .collect::<Vec<(VNCard, usize, usize)>>();

        if !poker_cards_on_table.is_empty() {
            let mut total_score: usize = 0;
            for r in 0..5 {
                let mut row_cards = poker_cards_on_table
                    .iter()
                    .filter(|(_, _col, row)| *row == r)
                    .map(|(card, col, _row)| (card.clone(), *col))
                    .collect::<Vec<(VNCard, usize)>>();
                // sort by position
                row_cards.sort_by(|a, b| a.1.cmp(&b.1));

                let row_cards = row_cards
                    .iter()
                    .map(|row| row.0.clone())
                    .collect::<Vec<VNCard>>();

                if row_cards.len() == 5 {
                    let (_, score) = check_poker_hand(row_cards);
                    total_score += score as usize;
                }
            }

            ew_refresh_ui.send(EventRefreshUI::PokerMenu(PokerMenuSettings {
                show_advance_button: game_state.n_draws == game_state.max_n_poker_draws,
                show_score: true,
                score: total_score,
            }));
        }
    }
}
