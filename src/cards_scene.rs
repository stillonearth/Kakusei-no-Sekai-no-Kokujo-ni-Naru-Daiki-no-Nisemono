use std::time::Duration;

use bevy::prelude::*;
use bevy_defer::AsyncCommandsExtension;
use bevy_defer::AsyncWorld;
use bevy_la_mesa::events::*;
use bevy_la_mesa::*;
use bevy_tweening::lens::TransformPositionLens;
use bevy_tweening::Animator;
use bevy_tweening::Tween;

use crate::cards_game::*;
use crate::cards_ui::*;
use crate::EventCardPositionHover;
use crate::EventCardPositionOut;
use crate::EventCardPositionPress;
use crate::NarrativeCardsHandle;

// ---------
// Resources
// ---------

#[derive(Resource, Default, Debug, PartialEq, Eq)]
pub(crate) enum GameType {
    #[default]
    Poker,
    Narrative,
    CardShop,
}

#[derive(Resource, Default)]
pub(crate) struct GameState {
    pub game_deck: Vec<VNCard>,
    pub collected_deck: Vec<VNCard>,
    pub game_type: GameType,
    pub max_n_poker_draws: usize,
    pub n_draws: usize,
    pub n_vn_node_scene_request: usize,
    pub n_vn_node: usize,
    pub narrative_conflicts: Vec<String>,
    pub narrative_plot_twists: Vec<String>,
    pub narrative_settings: Vec<String>,
    pub narrative_story_so_far: Vec<String>,
    pub poker_combinations: Vec<PokerCombination>,
    pub score: isize,
    pub ui_enable_play_hand: bool,
    pub ui_show_advance_button: bool,
}

// ------
// Events
// ------

#[derive(Event)]
pub(crate) struct EventStartPokerGame {}

#[derive(Event)]
pub(crate) struct EventStartNarrativeCardShop {}

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

#[derive(Component)]
pub(crate) struct CardShowcase {}

// --------------
// Event Handlers
// --------------

pub fn handle_card_press_cardplay(
    mut game_state: ResMut<GameState>,
    mut card_press: EventReader<CardPress>,
    mut ew_place_card_on_table: EventWriter<PlaceCardOnTable>,
    mut ew_handle_update_ui_game_state: EventWriter<EventUpdateGameStateUI>,
    mut q_cards: ParamSet<(
        Query<(Entity, &Card<VNCard>, &CardOnTable)>,
        Query<(Entity, &Card<VNCard>, &Hand)>,
    )>,
) {
    for event in card_press.read() {
        let p0 = q_cards.p0();
        let n_cards_on_table = p0.iter().len();
        if p0.get(event.entity).is_ok() {
            continue;
        }

        let p1 = q_cards.p1();

        if game_state.game_type == GameType::Narrative && n_cards_on_table < 2 {
            let card = p1.get(event.entity).unwrap().1;
            let card_type = card.data.metadata.card_type().unwrap_or_default();
            let effect = card.data.metadata.effect().unwrap_or_default();

            match card_type.as_str() {
                "setting" => {
                    game_state.narrative_settings.push(effect);
                }
                "plot twist" => {
                    game_state.narrative_plot_twists.push(effect);
                }
                "conflict" => {
                    game_state.narrative_conflicts.push(effect);
                }
                _ => {}
            }

            ew_place_card_on_table.send(PlaceCardOnTable {
                card_entity: event.entity,
                player: 1,
                marker: n_cards_on_table + 1,
            });

            if n_cards_on_table > 0 {
                game_state.ui_enable_play_hand = true;
            }

            ew_handle_update_ui_game_state.send(EventUpdateGameStateUI {});
        }
    }
}

pub fn handle_card_press_cardshop(
    mut game_state: ResMut<GameState>,
    mut card_press: EventReader<CardPress>,
    mut ew_discard_card_to_deck: EventWriter<DiscardCardToDeck>,
    q_cards_on_table: Query<(Entity, &Card<VNCard>, &CardOnTable)>,
    mut ew_update_game_state_ui: EventWriter<EventUpdateGameStateUI>,
    q_decks: Query<(Entity, &DeckArea)>,
) {
    for event in card_press.read() {
        if game_state.game_type == GameType::CardShop {
            let graveyard_deck_entity =
                q_decks.iter().find(|(_, deck)| deck.marker == 1).unwrap().0;

            let card = q_cards_on_table.get(event.entity).unwrap().1;
            let card_price = card.data.metadata.price().unwrap_or_default();

            if card_price <= (game_state.score as u16) {
                game_state.score -= card_price as isize;
                ew_discard_card_to_deck.send(DiscardCardToDeck {
                    card_entity: event.entity,
                    deck_entity: graveyard_deck_entity,
                });
                game_state.collected_deck.push(card.data.clone());
                ew_update_game_state_ui.send(EventUpdateGameStateUI {});
            }
        }
    }
}

pub(crate) fn handle_draw_to_hand(
    mut er_draw_deck: EventReader<DrawToHand>,
    mut game_state: ResMut<GameState>,
    mut ew_update_game_state_ui: EventWriter<EventUpdateGameStateUI>,
) {
    for _ in er_draw_deck.read() {
        game_state.n_draws += 1;
        // game_state.ui_enable_play_hand = true;
        ew_update_game_state_ui.send(EventUpdateGameStateUI {});
    }
}

pub(crate) fn handle_draw_to_table(
    mut er_draw_table: EventReader<DrawToTable>,
    mut game_state: ResMut<GameState>,
) {
    for _ in er_draw_table.read() {
        game_state.n_draws += 1;
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
    mut ew_play_hand_effect: EventWriter<EventPlayPokerHandEffect>,
    mut game_state: ResMut<GameState>,
    q_decks: Query<(Entity, &DeckArea)>,
) {
    for _ in er_play_hand.read() {
        let main_deck_entity = q_decks.iter().find(|(_, deck)| deck.marker == 1).unwrap().0;
        let graveyard_deck_entity = q_decks.iter().find(|(_, deck)| deck.marker == 1).unwrap().0;

        if game_state.game_type == GameType::Poker
            && game_state.n_draws == game_state.max_n_poker_draws
        {
            let poker_cards_on_table = q_cards
                .p0()
                .iter()
                .map(|(_, card, card_on_table)| {
                    (
                        card.data.clone(),
                        card_on_table.marker / 5,
                        card_on_table.marker % 5,
                    )
                })
                .collect::<Vec<(VNCard, usize, usize)>>();

            if poker_cards_on_table.is_empty() {
                continue;
            }

            let mut total_score = 0;
            for r in 0..5 {
                let mut row_cards = poker_cards_on_table
                    .iter()
                    .filter(|(_, row, _)| *row == r)
                    .map(|(card, _, col)| (card.clone(), *col))
                    .collect::<Vec<(VNCard, usize)>>();
                // sort by position
                row_cards.sort_by(|a, b| a.1.cmp(&b.1));
                let row_cards = row_cards
                    .iter()
                    .map(|row| row.0.clone())
                    .collect::<Vec<VNCard>>();

                let (_combination, score) = check_poker_hand(row_cards);

                total_score += score;
                game_state.score += score as isize;
                ew_play_hand_effect.send(EventPlayPokerHandEffect {
                    score: total_score as isize,
                });
            }

            for (entity, _, _) in q_cards.p1().iter() {
                ew_discard_card_to_deck.send(DiscardCardToDeck {
                    card_entity: entity,
                    deck_entity: main_deck_entity.clone(),
                });
            }
        }

        if game_state.game_type == GameType::Narrative {
            for (entity, _, _) in q_cards.p1().iter() {
                ew_discard_card_to_deck.send(DiscardCardToDeck {
                    card_entity: entity,
                    deck_entity: main_deck_entity.clone(),
                });
            }
        }

        if game_state.game_type == GameType::Poker {
            for (entity, _, _) in q_cards.p0().iter() {
                ew_discard_card_to_deck.send(DiscardCardToDeck {
                    card_entity: entity,
                    deck_entity: graveyard_deck_entity.clone(),
                });
            }
        }

        ew_align_cards_in_hand.send(AlignCardsInHand { player: 1 });
    }
}

pub(crate) fn handle_deck_rendered(
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut er_deck_rendered: EventReader<DeckRendered>,
    mut ew_shuffle: EventWriter<DeckShuffle>,
    mut ew_update_game_state_ui: EventWriter<EventUpdateGameStateUI>,
    q_decks: Query<(Entity, &DeckArea)>,
    q_cards: Query<(Entity, &Card<VNCard>, &Deck)>,
) {
    let deck_idle_time = 1.0;

    for _ in er_deck_rendered.read() {
        let main_deck_entity = q_decks.iter().find(|(_, deck)| deck.marker == 1).unwrap().0;
        let n_cards_on_table = q_cards
            .iter()
            .filter(|(_, _, deck)| deck.marker == 1)
            .count();
        let shuffle_animation_time = ((n_cards_on_table * 75) as f32) * 0.001;

        game_state.n_draws = 0;
        ew_shuffle.send(DeckShuffle {
            deck_entity: main_deck_entity.clone(),
        });
        match game_state.game_type {
            GameType::Narrative => {
                commands.spawn_task(move || async move {
                    AsyncWorld.sleep(deck_idle_time).await;
                    AsyncWorld.sleep(shuffle_animation_time).await;
                    AsyncWorld.send_event(DrawToHand {
                        deck_entity: main_deck_entity.clone(),
                        num_cards: 6,
                        player: 1,
                    })?;

                    Ok(())
                });
            }
            GameType::CardShop => {
                commands.spawn_task(move || async move {
                    AsyncWorld.sleep(deck_idle_time).await;
                    AsyncWorld.sleep(shuffle_animation_time).await;

                    let play_area_markers: Vec<usize> = (0..25).collect();
                    AsyncWorld.send_event(DrawToTable {
                        deck_entity: main_deck_entity.clone(),
                        play_area_markers,
                        player: 1,
                    })?;

                    Ok(())
                });
            }
            GameType::Poker => {
                commands.spawn_task(move || async move {
                    AsyncWorld.sleep(deck_idle_time).await;
                    AsyncWorld.sleep(shuffle_animation_time).await;
                    AsyncWorld.send_event(DrawToHand {
                        deck_entity: main_deck_entity.clone(),
                        num_cards: 1,
                        player: 1,
                    })?;

                    Ok(())
                });
            }
        }
    }
}

pub(crate) fn handle_start_poker_game(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut game_state: ResMut<GameState>,
    mut er_start_poker_game: EventReader<EventStartPokerGame>,
    mut ew_render_deck: EventWriter<RenderDeck<VNCard>>,
) {
    for _ in er_start_poker_game.read() {
        game_state.game_type = GameType::Poker;

        // Deck
        let deck_play_cards = commands
            .spawn((
                Mesh3d(meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10))),
                MeshMaterial3d(materials.add(Color::BLACK)),
                Transform::from_translation(Vec3::new(11.0, 0.0, 3.0))
                    .with_rotation(Quat::from_rotation_y(std::f32::consts::PI / 2.0)),
                Visibility::Hidden,
                DeckArea { marker: 1 },
                Name::new("Deck 1 -- Play Cards"),
            ))
            .id();

        let _ = commands
            .spawn((
                Mesh3d(meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10))),
                MeshMaterial3d(materials.add(Color::BLACK)),
                Transform::from_translation(Vec3::new(11.0, 0.0, -3.0))
                    .with_rotation(Quat::from_rotation_y(std::f32::consts::PI / 2.0)),
                Visibility::Hidden,
                DeckArea { marker: 2 },
                Name::new("Deck 2 -- Discarded Cards"),
            ))
            .id();

        // Hand Area -- LaMesa Plugin Draws Cards from deck to Hand
        commands.spawn((
            Name::new("HandArea - Player 1"),
            Transform::from_translation(Vec3::new(-0.5, 5.5, 5.8))
                .with_rotation(Quat::from_rotation_x(std::f32::consts::PI / 4.0)),
            HandArea { player: 1 },
        ));

        // Play Area
        for i in 0..5 {
            for j in 0..5 {
                let material = MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255)));

                commands
                    .spawn((
                        Mesh3d(
                            meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10)),
                        ),
                        material,
                        Transform::from_translation(Vec3::new(
                            -6.0 + 2.6 * (i as f32),
                            0.0,
                            6.0 - 3.6 * (j as f32),
                        )),
                        Visibility::Visible,
                        PlayArea {
                            marker: i * 5 + j,
                            player: 1,
                        },
                        Name::new(format!("Play Area {} {}", i, j)),
                        RayCastPickable,
                    ))
                    .observe(on_card_position_press)
                    .observe(on_card_position_over)
                    .observe(on_card_position_out);
            }
        }

        ew_render_deck.send(RenderDeck::<VNCard> {
            deck_entity: deck_play_cards,
            deck: load_poker_deck(),
        });
    }
}

fn on_card_position_press(
    click: Trigger<Pointer<Click>>,
    mut ew: EventWriter<EventCardPositionPress>,
) {
    ew.send(EventCardPositionPress {
        entity: click.entity(),
    });
}

fn on_card_position_over(
    click: Trigger<Pointer<Over>>,
    mut ew: EventWriter<EventCardPositionHover>,
) {
    ew.send(EventCardPositionHover {
        entity: click.entity(),
    });
}

fn on_card_position_out(click: Trigger<Pointer<Out>>, mut ew: EventWriter<EventCardPositionOut>) {
    ew.send(EventCardPositionOut {
        entity: click.entity(),
    });
}

pub(crate) fn handle_start_card_shop(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut game_state: ResMut<GameState>,
    mut er_start_card_shop: EventReader<EventStartNarrativeCardShop>,
    mut ew_render_deck: EventWriter<RenderDeck<VNCard>>,
    narrative_cards: Res<Assets<NarrativeCards>>,
    cards_handle: Res<NarrativeCardsHandle>,
) {
    for _ in er_start_card_shop.read() {
        game_state.game_type = GameType::CardShop;
        game_state.ui_show_advance_button = true;
        game_state.n_draws = 0;

        // Deck 1 - Shop Cards
        let deck_shop_cards = commands
            .spawn((
                Mesh3d(meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10))),
                MeshMaterial3d(materials.add(Color::BLACK)),
                Transform::from_translation(Vec3::new(8.5, 0.0, 0.0))
                    .with_rotation(Quat::from_rotation_y(std::f32::consts::PI / 2.0)),
                Visibility::Hidden,
                DeckArea { marker: 1 },
                Name::new("Deck 1 -- Shop Cards"),
            ))
            .id();

        // Deck 2 - All Cards
        let _ = commands
            .spawn((
                Mesh3d(meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10))),
                MeshMaterial3d(materials.add(Color::BLACK)),
                Transform::from_translation(Vec3::new(8.5, 0.0, 0.0))
                    .with_rotation(Quat::from_rotation_y(std::f32::consts::PI / 2.0)),
                Visibility::Hidden,
                DeckArea { marker: 2 },
                Name::new("Deck -- Bought Cards"),
            ))
            .id();

        // Play Area
        for i in 0..5 {
            for j in 0..5 {
                let material = MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255)));

                commands.spawn((
                    Mesh3d(meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10))),
                    material,
                    Transform::from_translation(Vec3::new(
                        -6.0 + 2.6 * (i as f32),
                        0.0,
                        6.0 - 3.6 * (j as f32),
                    )),
                    Visibility::Hidden,
                    PlayArea {
                        marker: i * 5 + j,
                        player: 1,
                    },
                    Name::new(format!("Play Area {} {}", i, j)),
                ));
            }
        }

        // Card Show Case
        let material = MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255)));
        commands.spawn((
            Mesh3d(meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10))),
            material,
            Transform::from_translation(Vec3::new(-5.3, 9.8, 5.8))
                .with_rotation(Quat::from_rotation_x(0.6)),
            Visibility::Hidden,
            CardShowcase {},
            Name::new("Card Show Case".to_string()),
        ));

        let narrative_cards = narrative_cards.get(cards_handle.id()).unwrap();
        let mut deck: Vec<VNCard> = vec![];
        for (i, narrative_card) in narrative_cards.iter().enumerate() {
            if i > 63 {
                break;
            }
            deck.push(VNCard {
                filename: format!("narrative-cards/card-{}.png", i + 1),
                metadata: VNCardMetadata::Narrative(
                    i + 1,
                    narrative_card.card_type.clone(),
                    narrative_card.genre.clone(),
                    narrative_card.name.clone(),
                    narrative_card.effect.clone(),
                    narrative_card.price,
                ),
            });
        }

        ew_render_deck.send(RenderDeck::<VNCard> {
            deck_entity: deck_shop_cards,
            deck,
        });
    }
}

pub(crate) fn handle_start_narrative_game(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut game_state: ResMut<GameState>,
    mut er_start_narrative_game: EventReader<EventStartNarrativeGame>,
    mut ew_render_deck: EventWriter<RenderDeck<VNCard>>,
) {
    for event in er_start_narrative_game.read() {
        game_state.game_type = GameType::Narrative;

        // Deck
        let deck_play_cards = commands
            .spawn((
                Mesh3d(meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10))),
                MeshMaterial3d(materials.add(Color::BLACK)),
                Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))
                    .with_rotation(Quat::from_rotation_y(std::f32::consts::PI / 2.0)),
                Visibility::Hidden,
                DeckArea { marker: 1 },
                Name::new("Deck 1 -- Play Cards"),
            ))
            .id();

        commands.spawn((
            Mesh3d(meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10))),
            MeshMaterial3d(materials.add(Color::BLACK)),
            Transform::from_translation(Vec3::new(4.0, 0.0, 0.0))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI / 2.0)),
            Visibility::Hidden,
            DeckArea { marker: 1 },
            Name::new("Deck 2 -- Play Cards"),
        ));

        // Table
        commands.spawn((
            Name::new("HandArea - Player 1"),
            Transform::from_translation(Vec3::new(0.0, 1.5, 5.8))
                .with_rotation(Quat::from_rotation_x(std::f32::consts::PI / 4.0)),
            HandArea { player: 1 },
        ));

        // Play Area
        let face_material = materials.add(Color::srgb_u8(124, 144, 255));

        commands.spawn((
            Mesh3d(meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10))),
            MeshMaterial3d(face_material.clone()),
            Transform::from_translation(Vec3::new(-7.6, 0.0, 7.0)),
            Visibility::Hidden,
            PlayArea {
                marker: 1,
                player: 1,
            },
            Name::new("Play Area 1"),
        ));

        commands.spawn((
            Mesh3d(meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10))),
            MeshMaterial3d(face_material.clone()),
            Transform::from_translation(Vec3::new(-7.6 + 3.05, 0.0, 7.0)),
            Visibility::Hidden,
            PlayArea {
                marker: 2,
                player: 1,
            },
            Name::new("Play Area 2"),
        ));

        commands.spawn((
            Mesh3d(meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10))),
            MeshMaterial3d(face_material.clone()),
            Transform::from_translation(Vec3::new(-7.6 + 3.05 * 2.0, 0.0, 7.0)),
            Visibility::Hidden,
            PlayArea {
                marker: 3,
                player: 1,
            },
            Name::new("Play Area 3"),
        ));

        commands.spawn((
            Mesh3d(meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10))),
            MeshMaterial3d(face_material.clone()),
            Transform::from_translation(Vec3::new(-7.6 + 3.05 * 3.0, 0.0, 7.0)),
            Visibility::Hidden,
            PlayArea {
                marker: 4,
                player: 1,
            },
            Name::new("Play Area 4"),
        ));

        commands.spawn((
            Mesh3d(meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10))),
            MeshMaterial3d(face_material.clone()),
            Transform::from_translation(Vec3::new(-7.6 + 3.05 * 4.0, 0.0, 7.0)),
            Visibility::Hidden,
            PlayArea {
                marker: 5,
                player: 1,
            },
            Name::new("Play Area 5"),
        ));

        ew_render_deck.send(RenderDeck::<VNCard> {
            deck_entity: deck_play_cards,
            deck: match event {
                EventStartNarrativeGame::Setting => {
                    filer_narrative_setting_deck(game_state.collected_deck.clone()).unwrap()
                }
                EventStartNarrativeGame::PlotTwist => {
                    filter_narrative_plot_twist_deck(game_state.collected_deck.clone()).unwrap()
                }
                EventStartNarrativeGame::Conflict => {
                    filter_narrative_conflict_deck(game_state.collected_deck.clone()).unwrap()
                }
            },
        });
    }
}

pub(crate) fn handle_card_on_table_hover(
    game_state: Res<GameState>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut hover: EventReader<CardHover>,
    mut cards_in_on_table: Query<(Entity, &mut Card<VNCard>, &CardOnTable, &Transform)>,
    mut q_card_showcase: Query<(
        Entity,
        &mut Visibility,
        &CardShowcase,
        &mut MeshMaterial3d<StandardMaterial>,
    )>,
) {
    hover.read().for_each(|hover| {
        if game_state.game_type == GameType::CardShop {
            if let Ok((_, card, hand, _transform)) = cards_in_on_table.get_mut(hover.entity) {
                if card.pickable && card.transform.is_some() {
                    let start_translation = card.transform.unwrap().translation;
                    let tween = Tween::new(
                        EaseFunction::QuadraticIn,
                        Duration::from_millis(100),
                        TransformPositionLens {
                            start: start_translation,
                            end: start_translation
                                + match hand.player {
                                    1 => Vec3::new(0., 0.7, 0.7),
                                    _ => Vec3::new(0., 0.7, 0.0),
                                },
                        },
                    );

                    for (_, mut visibility, _, mut material) in q_card_showcase.iter_mut() {
                        let face_texture = asset_server.load(card.data.front_image_filename());
                        let face_material = materials.add(StandardMaterial {
                            base_color_texture: Some(face_texture.clone()),
                            ..Default::default()
                        });
                        *material = MeshMaterial3d(face_material);
                        visibility.set_if_neq(Visibility::Visible);
                    }

                    commands.entity(hover.entity).insert(Animator::new(tween));
                }
            }
        }
    });
}

pub(crate) fn handle_card_on_table_out(
    game_state: Res<GameState>,
    mut commands: Commands,
    mut out: EventReader<CardOut>,
    mut cards_in_on_table: Query<(Entity, &mut Card<VNCard>, &CardOnTable, &Transform)>,
    mut q_card_showcase: Query<(Entity, &mut Visibility, &CardShowcase)>,
) {
    out.read().for_each(|hover| {
        if game_state.game_type == GameType::CardShop {
            if let Ok((_, card, _, transform)) = cards_in_on_table.get_mut(hover.entity) {
                if card.pickable && card.transform.is_some() {
                    let tween = Tween::new(
                        EaseFunction::QuadraticIn,
                        Duration::from_millis(100),
                        TransformPositionLens {
                            start: transform.translation,
                            end: card.transform.unwrap().translation,
                        },
                    );

                    for (_, mut visibility, _) in q_card_showcase.iter_mut() {
                        visibility.set_if_neq(Visibility::Hidden);
                    }

                    commands.entity(hover.entity).insert(Animator::new(tween));
                }
            }
        }
    });
}
