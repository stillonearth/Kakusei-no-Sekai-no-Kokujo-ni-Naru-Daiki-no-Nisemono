use bevy::prelude::*;
use bevy_la_mesa::events::*;
use bevy_la_mesa::*;

use crate::cards_game::*;
use crate::cards_ui::*;

// ---------
// Resources
// ---------

#[derive(Resource, Default, Debug, PartialEq, Eq)]
pub(crate) enum GameType {
    #[default]
    Poker,
    Narrative,
}

#[derive(Resource, Default)]
pub(crate) struct GameState {
    pub max_n_poker_draws: usize,
    pub n_draws: usize,
    pub n_vn_node: usize,
    pub n_vn_node_scene_request: usize,
    pub poker_combinations: Vec<PokerCombination>,
    pub poker_score: isize,
    pub ui_enable_play_hand: bool,
    pub ui_end_of_game: bool,
    pub narrative_settings: Vec<String>,
    pub narrative_plot_twists: Vec<String>,
    pub narrative_conflicts: Vec<String>,
    pub game_type: GameType,
    pub narrative_story_so_far: Vec<String>,
}

// ------
// Events
// ------

#[derive(Event)]
pub(crate) struct EventStartPokerGame {}

#[derive(Event)]
pub(crate) enum EventStartNarrativeGame {
    Setting,
    PlotTwist,
    Conflict,
}

#[derive(Event)]
pub(crate) struct EventPlayPokerHand {}

#[derive(Event)]
pub(crate) struct EventEndCardGame {}

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
    mut game_state: ResMut<GameState>,
    mut card_press: EventReader<CardPress>,
    mut ew_place_card_on_table: EventWriter<PlaceCardOnTable>,
    mut q_cards: ParamSet<(
        Query<(Entity, &Card<VNCard>, &CardOnTable)>,
        Query<(Entity, &Card<VNCard>, &Hand)>,
    )>,
) {
    for event in card_press.read() {
        let p0 = q_cards.p0();
        let n_cards_on_table = p0.iter().len();
        if p0.get(event.card_entity).is_ok() {
            continue;
        }

        let p1 = q_cards.p1();

        if game_state.game_type == GameType::Poker && n_cards_on_table < 5 {
            ew_place_card_on_table.send(PlaceCardOnTable {
                card_entity: event.card_entity,
                player: 1,
                marker: n_cards_on_table + 1,
            });
        }

        if game_state.game_type == GameType::Narrative && n_cards_on_table < 2 {
            let card = p1.get(event.card_entity).unwrap().1;
            let card_type = card.data.metadata.card_type().unwrap_or_default();
            let effect = card.data.metadata.effect().unwrap_or_default();
            match card_type.as_str() {
                "Setting" => {
                    game_state.narrative_settings.push(effect);
                }
                "Plot Twist" => {
                    game_state.narrative_plot_twists.push(effect);
                }
                "Conflict" => {
                    game_state.narrative_conflicts.push(effect);
                }
                _ => {}
            }

            ew_place_card_on_table.send(PlaceCardOnTable {
                card_entity: event.card_entity,
                player: 1,
                marker: n_cards_on_table + 1,
            });
        }
    }
}

pub(crate) fn handle_draw_hand(
    mut er_draw_deck: EventReader<DrawHand>,
    mut game_state: ResMut<GameState>,
    mut ew_update_game_state_ui: EventWriter<EventUpdateGameStateUI>,
) {
    for _ in er_draw_deck.read() {
        game_state.n_draws += 1;
        game_state.ui_enable_play_hand = true;
        ew_update_game_state_ui.send(EventUpdateGameStateUI {});
    }
}

pub(crate) fn handle_play_hand(
    mut q_cards: ParamSet<(
        Query<(Entity, &Card<VNCard>, &CardOnTable)>,
        Query<(Entity, &Card<VNCard>, &Hand)>,
    )>,
    mut er_play_hand: EventReader<EventPlayPokerHand>,
    mut ew_discard_card_to_deck: EventWriter<DiscardCardToDeck>,
    mut ew_align_cards_in_hand: EventWriter<AlignCardsInHand>,
    mut ew_update_game_state_ui: EventWriter<EventUpdateGameStateUI>,
    mut ew_play_hand_effect: EventWriter<EventPlayPokerHandEffect>,
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

        game_state.poker_combinations.push(combination.clone());
        game_state.poker_score += score as isize;
        game_state.ui_enable_play_hand = false;

        if game_state.game_type == GameType::Poker
            && game_state.n_draws == game_state.max_n_poker_draws
        {
            game_state.ui_end_of_game = true;

            for (entity, _, _) in q_cards.p1().iter() {
                ew_discard_card_to_deck.send(DiscardCardToDeck {
                    card_entity: entity,
                    deck_marker: 1,
                });
            }
        }

        if game_state.game_type == GameType::Narrative && game_state.n_draws == 1 {
            game_state.ui_end_of_game = true;

            for (entity, _, _) in q_cards.p1().iter() {
                ew_discard_card_to_deck.send(DiscardCardToDeck {
                    card_entity: entity,
                    deck_marker: 1,
                });
            }
        }

        if game_state.game_type == GameType::Poker {
            ew_play_hand_effect.send(EventPlayPokerHandEffect {
                combination: combination.clone(),
                score: score as isize,
            });
        }

        if game_state.game_type == GameType::Poker {
            for (entity, _, _) in q_cards.p0().iter() {
                ew_discard_card_to_deck.send(DiscardCardToDeck {
                    card_entity: entity,
                    deck_marker: 2,
                });
            }
        }

        ew_update_game_state_ui.send(EventUpdateGameStateUI {});
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
        game_state.n_draws = 0;
        game_state.ui_end_of_game = false;
        game_state.ui_enable_play_hand = false;
        ew_shuffle.send(DeckShuffle { deck_marker: 1 });
        ew_update_game_state_ui.send(EventUpdateGameStateUI {});
    }
}
