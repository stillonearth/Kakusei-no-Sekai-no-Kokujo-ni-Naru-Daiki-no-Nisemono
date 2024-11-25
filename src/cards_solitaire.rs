use bevy::prelude::*;
use bevy_la_mesa::{events::PlaceCardOnTable, Card, Hand, PlayArea};
use bevy_mod_picking::{events::*, prelude::*};

use crate::{cards_game::VNCard, GameState};

#[derive(Event)]
pub struct CardPositionHover {
    pub entity: Entity,
}

impl From<ListenerInput<Pointer<Over>>> for CardPositionHover {
    fn from(event: ListenerInput<Pointer<Over>>) -> Self {
        CardPositionHover {
            entity: event.target,
        }
    }
}

#[derive(Event)]
pub struct CardPositionOut {
    pub entity: Entity,
}

impl From<ListenerInput<Pointer<Out>>> for CardPositionOut {
    fn from(event: ListenerInput<Pointer<Out>>) -> Self {
        CardPositionOut {
            entity: event.target,
        }
    }
}

#[derive(Event)]
pub struct CardPositionPress {
    pub entity: Entity,
}

impl From<ListenerInput<Pointer<Down>>> for CardPositionPress {
    fn from(event: ListenerInput<Pointer<Down>>) -> Self {
        CardPositionPress {
            entity: event.target,
        }
    }
}
// Event Handlers
pub fn handle_card_position_hover(
    mut hover: EventReader<CardPositionHover>,
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
    mut hover: EventReader<CardPositionOut>,
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
    mut game_state: ResMut<GameState>,
    mut card_position_press: EventReader<CardPositionPress>,
    mut ew_place_card_on_table: EventWriter<PlaceCardOnTable>,
    q_cards_in_hand: Query<(Entity, &Card<VNCard>, &Hand)>,
    mut q_play_areas: Query<(Entity, &mut Visibility, &PlayArea)>,
) {
    for event in card_position_press.read() {
        if q_cards_in_hand.iter().len() == 0 {
            return;
        }

        let (card_entity, _, _) = q_cards_in_hand.single();

        if let Ok((entity, mut visibility, area)) = q_play_areas.get_mut(event.entity) {
            ew_place_card_on_table.send(PlaceCardOnTable {
                card_entity,
                player: 1,
                marker: area.marker,
            });

            *visibility = Visibility::Hidden;
        }
    }
}

// pub fn handle_card_out<T>(
//     mut commands: Commands,
//     mut out: EventReader<CardOut>,
//     mut query: Query<(Entity, &Card<T>, &Hand, &mut Transform)>,
// ) where
//     T: Send + Sync + Debug + 'static,
// {
//     out.read().for_each(|hover| {
//         if let Ok((_, card, _, transform)) = query.get_mut(hover.entity) {
//             if card.pickable && card.transform.is_some() {
//                 let tween = Tween::new(
//                     EaseFunction::QuadraticIn,
//                     Duration::from_millis(300),
//                     TransformPositionLens {
//                         start: transform.translation,
//                         end: card.transform.unwrap().translation,
//                     },
//                 );

//                 commands.entity(hover.entity).insert(Animator::new(tween));
//             }
//         }
//     });
// }
