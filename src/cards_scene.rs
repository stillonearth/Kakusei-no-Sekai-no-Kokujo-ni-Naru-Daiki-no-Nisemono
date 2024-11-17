use bevy::prelude::*;
use bevy_la_mesa::events::*;
use bevy_la_mesa::*;

use crate::cards_game::*;
use crate::cards_ui::*;

// ---------
// Resources
// ---------

#[derive(Resource, Default)]
pub(crate) struct GameState {
    pub combinations: Vec<PokerCombination>,
    pub score: isize,
    pub max_number_of_draws: usize,
    pub current_number_of_draws: usize,
    pub end_of_game: bool,
    pub enable_play_hand: bool,
    pub current_vn_node: usize,
}

// ------
// Events
// ------

#[derive(Event)]
pub(crate) struct EventPlayHand {}

#[derive(Event)]
pub(crate) struct EventEndGame {}

// -------
// Systems
// -------

pub(crate) fn setup_card_scene(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Deck
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10)),
            material: materials.add(Color::BLACK),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI / 2.0)),
            visibility: Visibility::Hidden,
            ..default()
        },
        DeckArea { marker: 1 },
        Name::new("Deck 1 -- Play Cards"),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10)),
            material: materials.add(Color::BLACK),
            transform: Transform::from_translation(Vec3::new(4.0, 0.0, 0.0))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI / 2.0)),
            visibility: Visibility::Hidden,
            ..default()
        },
        DeckArea { marker: 2 },
        Name::new("Deck 2 -- Play Cards"),
    ));

    // Table
    commands.spawn((
        Name::new("HandArea - Player 1"),
        TransformBundle {
            local: Transform::from_translation(Vec3::new(0.0, 1.5, 5.8))
                .with_rotation(Quat::from_rotation_x(std::f32::consts::PI / 4.0)),
            ..default()
        },
        HandArea { player: 1 },
    ));

    // Play Area
    let face_material = materials.add(Color::srgb_u8(124, 144, 255));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10)),
            material: face_material.clone(),
            transform: Transform::from_translation(Vec3::new(-7.6, 0.0, 7.0)),
            visibility: Visibility::Hidden,
            ..default()
        },
        PlayArea {
            marker: 1,
            player: 1,
        },
        Name::new("Play Area 1"),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10)),
            material: face_material.clone(),
            transform: Transform::from_translation(Vec3::new(-7.6 + 3.05, 0.0, 7.0)),
            visibility: Visibility::Hidden,
            ..default()
        },
        PlayArea {
            marker: 2,
            player: 1,
        },
        Name::new("Play Area 2"),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10)),
            material: face_material.clone(),
            transform: Transform::from_translation(Vec3::new(-7.6 + 3.05 * 2.0, 0.0, 7.0)),
            visibility: Visibility::Hidden,
            ..default()
        },
        PlayArea {
            marker: 3,
            player: 1,
        },
        Name::new("Play Area 3"),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10)),
            material: face_material.clone(),
            transform: Transform::from_translation(Vec3::new(-7.6 + 3.05 * 3.0, 0.0, 7.0)),
            visibility: Visibility::Hidden,
            ..default()
        },
        PlayArea {
            marker: 4,
            player: 1,
        },
        Name::new("Play Area 4"),
    ));

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10)),
            material: face_material.clone(),
            transform: Transform::from_translation(Vec3::new(-7.6 + 3.05 * 4.0, 0.0, 7.0)),
            visibility: Visibility::Hidden,
            ..default()
        },
        PlayArea {
            marker: 5,
            player: 1,
        },
        Name::new("Play Area 5"),
    ));
}

// --------------
// Event Handlers
// --------------

pub fn handle_card_press(
    mut card_press: EventReader<CardPress>,
    mut ew_place_card_on_table: EventWriter<PlaceCardOnTable>,
    q_cards_on_table: Query<(Entity, &CardOnTable)>,
) {
    for event in card_press.read() {
        if q_cards_on_table.get(event.card_entity).is_ok() {
            continue;
        }

        let n_cards_on_table = q_cards_on_table.iter().len();

        ew_place_card_on_table.send(PlaceCardOnTable {
            card_entity: event.card_entity,
            player: 1,
            marker: n_cards_on_table + 1,
        });
    }
}

pub(crate) fn handle_draw_hand(
    mut er_draw_deck: EventReader<DrawHand>,
    mut game_state: ResMut<GameState>,
    mut ew_update_game_state_ui: EventWriter<EventUpdateGameStateUI>,
) {
    for _ in er_draw_deck.read() {
        game_state.current_number_of_draws += 1;
        game_state.enable_play_hand = true;
        ew_update_game_state_ui.send(EventUpdateGameStateUI {});
    }
}

pub(crate) fn handle_play_hand(
    mut q_cards: ParamSet<(
        Query<(Entity, &Card<VNCard>, &CardOnTable)>,
        Query<(Entity, &Card<VNCard>, &Hand)>,
    )>,
    mut er_play_hand: EventReader<EventPlayHand>,
    mut ew_discard_card_to_deck: EventWriter<DiscardCardToDeck>,
    mut ew_align_cards_in_hand: EventWriter<AlignCardsInHand>,
    mut ew_update_game_state_ui: EventWriter<EventUpdateGameStateUI>,
    mut ew_play_hand_effect: EventWriter<EventPlayHandEffect>,
    mut game_state: ResMut<GameState>,
) {
    for _ in er_play_hand.read() {
        let poker_cards_on_table = q_cards
            .p0()
            .iter()
            .map(|(_, card, _)| card.data.clone())
            .collect::<Vec<VNCard>>();

        if poker_cards_on_table.is_empty() {
            continue;
        }

        let (combination, score) = check_poker_hand(poker_cards_on_table);

        game_state.combinations.push(combination.clone());
        game_state.score += score as isize;
        game_state.enable_play_hand = false;

        if game_state.current_number_of_draws == game_state.max_number_of_draws {
            game_state.end_of_game = true;

            for (entity, _, _) in q_cards.p1().iter() {
                ew_discard_card_to_deck.send(DiscardCardToDeck {
                    card_entity: entity,
                    deck_marker: 1,
                });
            }
        }

        ew_play_hand_effect.send(EventPlayHandEffect {
            combination: combination.clone(),
            score: score as isize,
        });

        ew_update_game_state_ui.send(EventUpdateGameStateUI {});

        for (entity, _, _) in q_cards.p0().iter() {
            ew_discard_card_to_deck.send(DiscardCardToDeck {
                card_entity: entity,
                deck_marker: 2,
            });
        }

        ew_align_cards_in_hand.send(AlignCardsInHand { player: 1 });
    }
}

pub(crate) fn handle_deck_rendered_card_game(
    mut er_deck_rendered: EventReader<DeckRendered>,
    mut ew_shuffle: EventWriter<DeckShuffle>,
    mut ew_update_game_state_ui: EventWriter<EventUpdateGameStateUI>,
    mut game_state: ResMut<GameState>,
) {
    for _ in er_deck_rendered.read() {
        game_state.current_number_of_draws = 0;
        game_state.end_of_game = false;
        game_state.enable_play_hand = false;
        ew_shuffle.send(DeckShuffle { deck_marker: 1 });
        ew_update_game_state_ui.send(EventUpdateGameStateUI {});
    }
}
