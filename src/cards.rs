use bevy::utils::HashMap;
use bevy::{color::palettes::css::*, prelude::*};
use bevy_la_mesa::events::*;
use bevy_la_mesa::*;

#[derive(Default, Clone, Debug)]
pub(crate) struct PokerCard {
    pub(crate) value: u8,
    pub(crate) suit: String,
    pub(crate) filename: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub(crate) struct Chip {}

#[allow(clippy::vec_init_then_push)]
pub(crate) fn load_poker_deck() -> Vec<PokerCard> {
    let mut deck: Vec<PokerCard> = vec![];

    // Clubs
    deck.push(PokerCard {
        value: 2,
        suit: "Clubs".to_string(),
        filename: "cards/Clubs_2.png".to_string(),
    });
    deck.push(PokerCard {
        value: 3,
        suit: "Clubs".to_string(),
        filename: "cards/Clubs_3.png".to_string(),
    });
    deck.push(PokerCard {
        value: 4,
        suit: "Clubs".to_string(),
        filename: "cards/Clubs_4.png".to_string(),
    });
    deck.push(PokerCard {
        value: 5,
        suit: "Clubs".to_string(),
        filename: "cards/Clubs_5.png".to_string(),
    });
    deck.push(PokerCard {
        value: 6,
        suit: "Clubs".to_string(),
        filename: "cards/Clubs_6.png".to_string(),
    });
    deck.push(PokerCard {
        value: 7,
        suit: "Clubs".to_string(),
        filename: "cards/Clubs_7.png".to_string(),
    });
    deck.push(PokerCard {
        value: 8,
        suit: "Clubs".to_string(),
        filename: "cards/Clubs_8.png".to_string(),
    });
    deck.push(PokerCard {
        value: 9,
        suit: "Clubs".to_string(),
        filename: "cards/Clubs_9.png".to_string(),
    });
    deck.push(PokerCard {
        value: 10,
        suit: "Clubs".to_string(),
        filename: "cards/Clubs_10.png".to_string(),
    });
    deck.push(PokerCard {
        value: 11,
        suit: "Clubs".to_string(),
        filename: "cards/Clubs_J.png".to_string(),
    });
    deck.push(PokerCard {
        value: 12,
        suit: "Clubs".to_string(),
        filename: "cards/Clubs_Q.png".to_string(),
    });
    deck.push(PokerCard {
        value: 13,
        suit: "Clubs".to_string(),
        filename: "cards/Clubs_K.png".to_string(),
    });
    deck.push(PokerCard {
        value: 134,
        suit: "Clubs".to_string(),
        filename: "cards/Clubs_ACE.png".to_string(),
    });

    // Diamonds
    deck.push(PokerCard {
        value: 2,
        suit: "Diamonds".to_string(),
        filename: "cards/Diamonds_2.png".to_string(),
    });
    deck.push(PokerCard {
        value: 3,
        suit: "Diamonds".to_string(),
        filename: "cards/Diamonds_3.png".to_string(),
    });
    deck.push(PokerCard {
        value: 4,
        suit: "Diamonds".to_string(),
        filename: "cards/Diamonds_4.png".to_string(),
    });
    deck.push(PokerCard {
        value: 5,
        suit: "Diamonds".to_string(),
        filename: "cards/Diamonds_5.png".to_string(),
    });
    deck.push(PokerCard {
        value: 6,
        suit: "Diamonds".to_string(),
        filename: "cards/Diamonds_6.png".to_string(),
    });
    deck.push(PokerCard {
        value: 7,
        suit: "Diamonds".to_string(),
        filename: "cards/Diamonds_7.png".to_string(),
    });
    deck.push(PokerCard {
        value: 8,
        suit: "Diamonds".to_string(),
        filename: "cards/Diamonds_8.png".to_string(),
    });
    deck.push(PokerCard {
        value: 9,
        suit: "Diamonds".to_string(),
        filename: "cards/Diamonds_9.png".to_string(),
    });
    deck.push(PokerCard {
        value: 10,
        suit: "Diamonds".to_string(),
        filename: "cards/Diamonds_10.png".to_string(),
    });
    deck.push(PokerCard {
        value: 11,
        suit: "Diamonds".to_string(),
        filename: "cards/Diamonds_J.png".to_string(),
    });
    deck.push(PokerCard {
        value: 12,
        suit: "Diamonds".to_string(),
        filename: "cards/Diamonds_Q.png".to_string(),
    });
    deck.push(PokerCard {
        value: 13,
        suit: "Diamonds".to_string(),
        filename: "cards/Diamonds_K.png".to_string(),
    });
    deck.push(PokerCard {
        value: 134,
        suit: "Diamonds".to_string(),
        filename: "cards/Diamonds_ACE.png".to_string(),
    });

    // Hearts
    deck.push(PokerCard {
        value: 2,
        suit: "Hearts".to_string(),
        filename: "cards/Hearts_2.png".to_string(),
    });
    deck.push(PokerCard {
        value: 3,
        suit: "Hearts".to_string(),
        filename: "cards/Hearts_3.png".to_string(),
    });
    deck.push(PokerCard {
        value: 4,
        suit: "Hearts".to_string(),
        filename: "cards/Hearts_4.png".to_string(),
    });
    deck.push(PokerCard {
        value: 5,
        suit: "Hearts".to_string(),
        filename: "cards/Hearts_5.png".to_string(),
    });
    deck.push(PokerCard {
        value: 6,
        suit: "Hearts".to_string(),
        filename: "cards/Hearts_6.png".to_string(),
    });
    deck.push(PokerCard {
        value: 7,
        suit: "Hearts".to_string(),
        filename: "cards/Hearts_7.png".to_string(),
    });
    deck.push(PokerCard {
        value: 8,
        suit: "Hearts".to_string(),
        filename: "cards/Hearts_8.png".to_string(),
    });
    deck.push(PokerCard {
        value: 9,
        suit: "Hearts".to_string(),
        filename: "cards/Hearts_9.png".to_string(),
    });
    deck.push(PokerCard {
        value: 10,
        suit: "Hearts".to_string(),
        filename: "cards/Hearts_10.png".to_string(),
    });
    deck.push(PokerCard {
        value: 11,
        suit: "Hearts".to_string(),
        filename: "cards/Hearts_J.png".to_string(),
    });
    deck.push(PokerCard {
        value: 12,
        suit: "Hearts".to_string(),
        filename: "cards/Hearts_Q.png".to_string(),
    });
    deck.push(PokerCard {
        value: 13,
        suit: "Hearts".to_string(),
        filename: "cards/Hearts_K.png".to_string(),
    });
    deck.push(PokerCard {
        value: 134,
        suit: "Hearts".to_string(),
        filename: "cards/Hearts_ACE.png".to_string(),
    });

    // Spades
    deck.push(PokerCard {
        value: 2,
        suit: "Spades".to_string(),
        filename: "cards/Spades_2.png".to_string(),
    });
    deck.push(PokerCard {
        value: 3,
        suit: "Spades".to_string(),
        filename: "cards/Spades_3.png".to_string(),
    });
    deck.push(PokerCard {
        value: 4,
        suit: "Spades".to_string(),
        filename: "cards/Spades_4.png".to_string(),
    });
    deck.push(PokerCard {
        value: 5,
        suit: "Spades".to_string(),
        filename: "cards/Spades_5.png".to_string(),
    });
    deck.push(PokerCard {
        value: 6,
        suit: "Spades".to_string(),
        filename: "cards/Spades_6.png".to_string(),
    });
    deck.push(PokerCard {
        value: 7,
        suit: "Spades".to_string(),
        filename: "cards/Spades_7.png".to_string(),
    });
    deck.push(PokerCard {
        value: 8,
        suit: "Spades".to_string(),
        filename: "cards/Spades_8.png".to_string(),
    });
    deck.push(PokerCard {
        value: 9,
        suit: "Spades".to_string(),
        filename: "cards/Spades_9.png".to_string(),
    });
    deck.push(PokerCard {
        value: 10,
        suit: "Spades".to_string(),
        filename: "cards/Spades_10.png".to_string(),
    });
    deck.push(PokerCard {
        value: 11,
        suit: "Spades".to_string(),
        filename: "cards/Spades_J.png".to_string(),
    });
    deck.push(PokerCard {
        value: 12,
        suit: "Spades".to_string(),
        filename: "cards/Spades_Q.png".to_string(),
    });
    deck.push(PokerCard {
        value: 13,
        suit: "Spades".to_string(),
        filename: "cards/Spades_K.png".to_string(),
    });
    deck.push(PokerCard {
        value: 134,
        suit: "Spades".to_string(),
        filename: "cards/Spades_ACE.png".to_string(),
    });

    deck
}

impl CardMetadata for PokerCard {
    type Output = PokerCard;

    fn filename(&self) -> String {
        self.filename.clone()
    }
}

// UI

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

#[derive(Component)]
pub(crate) struct ButtonShuffleDeck;

#[derive(Component)]
pub(crate) struct ButtonDrawHand;

#[derive(Component)]
pub(crate) struct ButtonPlayHand;

pub(crate) fn setup_ui(mut commands: Commands, _sasset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Px(65.0),
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            Name::new("UI"),
        ))
        .with_children(|parent| {
            // Shuffle
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(350.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        border_radius: BorderRadius::MAX,
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonShuffleDeck,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "shuffle deck",
                        TextStyle {
                            // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });

            // Draw hands
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(350.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        border_radius: BorderRadius::MAX,
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonDrawHand,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "draw hand",
                        TextStyle {
                            // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });

            // Draw hands
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(350.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        border_color: BorderColor(Color::BLACK),
                        border_radius: BorderRadius::MAX,
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ButtonPlayHand,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "play hand",
                        TextStyle {
                            // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 40.0,
                            color: Color::srgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        });
}

pub fn handle_buttons(
    mut set: ParamSet<(
        Query<
            (
                &Interaction,
                &mut BackgroundColor,
                &mut BorderColor,
                &Children,
                &ButtonShuffleDeck,
            ),
            (Changed<Interaction>, With<Button>),
        >,
        Query<
            (
                &Interaction,
                &mut BackgroundColor,
                &mut BorderColor,
                &Children,
                &ButtonDrawHand,
            ),
            (Changed<Interaction>, With<Button>),
        >,
        Query<
            (
                &Interaction,
                &mut BackgroundColor,
                &mut BorderColor,
                &Children,
                &ButtonPlayHand,
            ),
            (Changed<Interaction>, With<Button>),
        >,
    )>,
    decks: Query<(Entity, &DeckArea)>,
    mut text_query: Query<&mut Text>,
    mut ew_shuffle: EventWriter<DeckShuffle>,
    mut ew_draw: EventWriter<DrawHand>,
    mut ew_play_hand: EventWriter<PlayHand>,
) {
    if decks.iter().count() == 0 {
        return;
    }

    for (interaction, mut color, mut border_color, children, _) in &mut set.p0().iter_mut() {
        let mut _text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                // text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();

                ew_shuffle.send(DeckShuffle { deck_marker: 1 });
            }
            Interaction::Hovered => {
                // text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                // text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }

    for (interaction, mut color, mut border_color, children, _) in &mut set.p1().iter_mut() {
        let mut _text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                // text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();

                ew_draw.send(DrawHand {
                    deck_marker: 1,
                    num_cards: 5,
                    player: 1,
                });
            }
            Interaction::Hovered => {
                // text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                // text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }

    for (interaction, mut color, mut border_color, children, _) in &mut set.p2().iter_mut() {
        let mut _text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                // text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();

                ew_play_hand.send(PlayHand {});
            }
            Interaction::Hovered => {
                // text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                // text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub(crate) fn setup_card_scene(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // light
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

pub fn handle_card_press(
    mut card_press: EventReader<CardPress>,
    mut ew_place_card_on_table: EventWriter<PlaceCardOnTable>,
    q_cards_on_table: Query<(Entity, &CardOnTable)>,
) {
    for event in card_press.read() {
        let n_cards_on_table = q_cards_on_table.iter().len();

        ew_place_card_on_table.send(PlaceCardOnTable {
            card_entity: event.card_entity,
            player: 1,
            marker: n_cards_on_table + 1,
        });
    }
}

// Combinations

fn is_flush(cards: &[PokerCard]) -> bool {
    let suit = &cards[0].suit;
    cards.iter().all(|card| card.suit == *suit)
}

fn is_straight(values: &[u8]) -> bool {
    values.windows(2).all(|w| w[1] == w[0] + 1) || (values.contains(&2) && values.contains(&14))
}

fn get_value_counts(cards: &[PokerCard]) -> HashMap<u8, usize> {
    let mut counts = HashMap::new();
    for card in cards {
        *counts.entry(card.value).or_insert(0) += 1;
    }
    counts
}

fn is_royal_flush(cards: &[PokerCard]) -> bool {
    if !is_flush(cards) {
        return false;
    }
    let values: Vec<u8> = cards.iter().map(|card| card.value).collect();
    values.contains(&10)
        && values.contains(&11)
        && values.contains(&12)
        && values.contains(&13)
        && values.contains(&14)
}

fn is_straight_flush(cards: &[PokerCard]) -> bool {
    is_flush(cards)
        && is_straight(&cards.iter().map(|card| card.value).collect::<Vec<u8>>())
        && cards.len() == 5
}

fn four_of_a_kind(cards: &[PokerCard]) -> (bool, u8) {
    let mut counts = HashMap::new();
    for card in cards {
        *counts.entry(card.value).or_insert(0) += 1;
    }
    let mut score = 0;
    if let Some(&value) = counts.values().find(|&&v| v == 4) {
        score = value;
        (true, score)
    } else {
        (false, score)
    }
}

fn full_house(cards: &[PokerCard]) -> (bool, u8) {
    let mut counts = HashMap::new();
    for card in cards {
        *counts.entry(card.value).or_insert(0) += 1;
    }
    let values: Vec<_> = counts.values().collect();
    if values.contains(&&3) && values.contains(&&2) {
        (true, *counts.keys().max().unwrap())
    } else {
        (false, 0)
    }
}

fn straight(cards: &[PokerCard]) -> bool {
    is_straight(&cards.iter().map(|card| card.value).collect::<Vec<u8>>())
}

fn three_of_a_kind(cards: &[PokerCard]) -> (bool, u8) {
    let mut counts = HashMap::new();
    for card in cards {
        *counts.entry(card.value).or_insert(0) += 1;
    }
    let mut score = 0;
    if let Some(&value) = counts.values().find(|&&v| v == 3) {
        score = value;
        (true, score)
    } else {
        (false, score)
    }
}

fn two_pair(cards: &[PokerCard]) -> (bool, u8) {
    let mut counts = HashMap::new();
    for card in cards {
        *counts.entry(card.value).or_insert(0) += 1;
    }
    let values: Vec<_> = counts.values().collect();
    if values.iter().filter(|&&v| *v == 2).count() == 2 {
        (true, *counts.keys().max().unwrap())
    } else {
        (false, 0)
    }
}

fn one_pair(cards: &[PokerCard]) -> (bool, u8) {
    let mut counts = HashMap::new();
    for card in cards {
        *counts.entry(card.value).or_insert(0) += 1;
    }
    let mut score = 0;
    if let Some(&value) = counts.values().find(|&&v| v == 2) {
        score = value;
        (true, score)
    } else {
        (false, score)
    }
}

fn high_card(cards: &[PokerCard]) -> u8 {
    cards.iter().map(|card| card.value).max().unwrap()
}

fn check_poker_hand(cards: Vec<PokerCard>) -> (String, u8) {
    let sorted_cards = {
        let mut cards = cards;
        cards.sort_by_key(|k| k.value);
        cards
    };

    if is_royal_flush(&sorted_cards) {
        return ("Royal Flush".to_string(), 10);
    } else if is_straight_flush(&sorted_cards) {
        return ("Straight Flush".to_string(), 9);
    }

    let (four_of_a_kind, score) = four_of_a_kind(&sorted_cards);
    if four_of_a_kind {
        return ("Four of a Kind".to_string(), 8 + score);
    }

    let (full_house, score) = full_house(&sorted_cards);
    if full_house {
        return ("Full House".to_string(), 7 + score);
    }

    if is_flush(&sorted_cards) {
        return ("Flush".to_string(), 6 + high_card(&sorted_cards));
    }

    if straight(&sorted_cards) {
        return (
            "Straight".to_string(),
            5 + sorted_cards.last().unwrap().value,
        );
    }

    let (three_of_a_kind, score) = three_of_a_kind(&sorted_cards);
    if three_of_a_kind {
        return ("Three of a Kind".to_string(), 4 + score);
    }

    let (two_pair, score) = two_pair(&sorted_cards);
    if two_pair {
        return ("Two Pair".to_string(), 3 + score);
    }

    let (one_pair, score) = one_pair(&sorted_cards);
    if one_pair {
        return ("One Pair".to_string(), 2 + score);
    }

    ("High Card".to_string(), 1 + high_card(&sorted_cards))
}

// Events

#[derive(Event)]
pub(crate) struct PlayHand {}

pub(crate) fn handle_play_hand(
    mut er_play_hand: EventReader<PlayHand>,
    mut ew_place_card_off_table: EventWriter<PlaceCardOffTable>,
    cards_on_table: Query<(Entity, &Card<PokerCard>, &CardOnTable)>,
) {
    for _ in er_play_hand.read() {
        let poker_cards_on_table = cards_on_table
            .iter()
            .map(|(_, card, _)| card.data.clone())
            .collect::<Vec<PokerCard>>();

        let (hand_type, score) = check_poker_hand(poker_cards_on_table);

        println!("hand: {}\tscore: {}", hand_type, score);

        for (entity, _, _) in cards_on_table.iter() {
            ew_place_card_off_table.send(PlaceCardOffTable {
                card_entity: entity,
                deck_marker: 1,
            });
        }
    }
}
