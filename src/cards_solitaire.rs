use bevy::prelude::*;
use bevy_la_mesa::{
    events::{DrawHand, PlaceCardOnTable},
    Card, Hand, PlayArea,
};
use bevy_mod_picking::{events::*, prelude::*};
use pecs::prelude::*;

use crate::{cards_game::VNCard, GameState};

#[derive(Event)]
pub struct EventCardPositionHover {
    pub entity: Entity,
}

impl From<ListenerInput<Pointer<Over>>> for EventCardPositionHover {
    fn from(event: ListenerInput<Pointer<Over>>) -> Self {
        EventCardPositionHover {
            entity: event.target,
        }
    }
}

#[derive(Event)]
pub struct EventCardPositionOut {
    pub entity: Entity,
}

impl From<ListenerInput<Pointer<Out>>> for EventCardPositionOut {
    fn from(event: ListenerInput<Pointer<Out>>) -> Self {
        EventCardPositionOut {
            entity: event.target,
        }
    }
}

#[derive(Event)]
pub struct EventCardPositionPress {
    pub entity: Entity,
}

impl From<ListenerInput<Pointer<Down>>> for EventCardPositionPress {
    fn from(event: ListenerInput<Pointer<Down>>) -> Self {
        EventCardPositionPress {
            entity: event.target,
        }
    }
}
// Event Handlers
pub fn handle_card_position_hover(
    mut hover: EventReader<EventCardPositionHover>,
    mut query: Query<(Entity, &mut Handle<StandardMaterial>, &PlayArea)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    hover.read().for_each(|hover| {
        if let Ok((_, mut material, _)) = query.get_mut(hover.entity) {
            *material = materials.add(Color::srgb_u8(255, 144, 255));
        }
    });
}

pub fn handle_card_position_out(
    mut hover: EventReader<EventCardPositionOut>,
    mut query: Query<(Entity, &mut Handle<StandardMaterial>, &PlayArea)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    hover.read().for_each(|hover| {
        if let Ok((_, mut material, _)) = query.get_mut(hover.entity) {
            *material = materials.add(Color::srgb_u8(124, 144, 255));
        }
    });
}

pub fn handle_card_position_press(
    mut commands: Commands,
    game_state: ResMut<GameState>,
    mut card_position_press: EventReader<EventCardPositionPress>,
    mut ew_place_card_on_table: EventWriter<PlaceCardOnTable>,
    q_cards_in_hand: Query<(Entity, &Card<VNCard>, &Hand)>,
    mut q_play_areas: Query<(Entity, &mut Visibility, &PlayArea)>,

    time: Res<Time>,
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
                // a heck to invoke system next frame

                let start = time.elapsed_seconds();
                commands
                    .promise(|| start)
                    .then(asyn!(state => {
                        state.asyn().timeout(0.001)
                    }))
                    .then(asyn!(_, mut ew_draw: EventWriter<DrawHand>  => {
                        ew_draw.send(DrawHand {
                            deck_marker: 1,
                            num_cards: 1,
                            player: 1,
                        });
                    }));
            }

            *visibility = Visibility::Hidden;
        }
    }
}
