use bevy::{color::palettes::css::*, prelude::*};
use bevy_la_mesa::events::*;
use bevy_la_mesa::*;

use bevy_novel::events::EventSwitchNextNode;

use crate::{cards_game::*, EventEndCardGame, EventPlayPokerHand, GameState, GameType};

// UI
const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

// ------
// Events
// ------

#[derive(Event)]
pub(crate) struct EventPlayPokerHandEffect {
    pub score: isize,
}

// ----------
// Components
// ----------

#[derive(Component)]
pub(crate) struct UIRootNode;

#[derive(Component)]
pub(crate) struct UIButtonShuffleDeck;

#[derive(Component)]
pub(crate) struct UIButtonDrawToHand;

#[derive(Component)]
pub(crate) struct UIButtonPlayHand;

#[derive(Component)]
pub(crate) struct UIButtonAdvance;

#[derive(Component)]
pub(crate) struct UILabelCenterScreen;

#[derive(Component)]
pub(crate) struct UILabelGameState;

#[derive(Component)]
pub(crate) struct UINodeFullScreen;

#[derive(Component)]
pub(crate) struct UINodeGameState;

#[derive(Event)]
pub(crate) struct EventHideUIOverlay {}

#[derive(Event)]
pub(crate) struct EventUpdateGameStateUI {}

// -------
// Systems
// -------

pub(crate) fn handle_ui_update_game_state(
    game_state: Res<GameState>,
    mut er_update_game_state_ui: EventReader<EventUpdateGameStateUI>,
    mut q_game_state_label: Query<(Entity, &mut Text, &UILabelGameState)>,
    mut paramset_buttons: ParamSet<(
        Query<(Entity, &mut Visibility, &UIButtonAdvance)>,
        Query<(Entity, &mut Visibility, &UIButtonDrawToHand)>,
        Query<(Entity, &mut Visibility, &UIButtonPlayHand)>,
    )>,
) {
    for _ in er_update_game_state_ui.read() {
        for (_, mut text, _) in q_game_state_label.iter_mut() {
            let _cominations = game_state
                .poker_combinations
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
                .join("\n");

            *text = Text::new(format!("${}", game_state.score,));
        }

        for (_, mut visibility, _) in paramset_buttons.p0().iter_mut() {
            *visibility = match game_state.ui_show_advance_button {
                true => Visibility::Visible,
                false => Visibility::Hidden,
            }
        }

        for (_, mut visibility, _) in paramset_buttons.p1().iter_mut() {
            *visibility = Visibility::Hidden;
        }

        for (_, mut visibility, _) in paramset_buttons.p2().iter_mut() {
            *visibility = match !game_state.ui_show_advance_button && game_state.ui_enable_play_hand
            {
                true => Visibility::Visible,
                false => Visibility::Hidden,
            }
        }
    }
}

pub(crate) fn handle_hide_ui_overlay(
    mut q_overlay: Query<(&mut Visibility, &UINodeFullScreen)>,
    mut er_hide_ui_overlay: EventReader<EventHideUIOverlay>,
) {
    for _ in er_hide_ui_overlay.read() {
        for (mut visibility, _) in q_overlay.iter_mut() {
            *visibility = Visibility::Hidden;
        }
    }
}

pub(crate) fn handle_play_hand_effect(
    game_state: Res<GameState>,
    mut er_play_hand_effect: EventReader<EventPlayPokerHandEffect>,
    mut q_ui: ParamSet<(
        Query<(&mut Text, &UILabelCenterScreen)>,
        Query<(&mut Visibility, &UINodeFullScreen)>,
    )>,
    mut ew_hide_ui_overlay: EventWriter<EventHideUIOverlay>,
) {
    for event in er_play_hand_effect.read() {
        if game_state.game_type == GameType::Poker {
            for (mut text, _) in q_ui.p0().iter_mut() {
                *text = Text::new(format!("${}", event.score));
            }

            for (mut visibility, _) in q_ui.p1().iter_mut() {
                *visibility = Visibility::Visible;
                ew_hide_ui_overlay.send(EventHideUIOverlay {});
            }
        }
    }
}

pub(crate) fn handle_deck_rendered_card_ui(
    mut er_deck_rendered: EventReader<DeckRendered>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for _ in er_deck_rendered.read() {
        commands
            .spawn((
                Name::new("UI Node Game State"),
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(20.0),
                    left: Val::Px(20.0),
                    ..default()
                },
                UIRootNode {},
                UINodeGameState {},
            ))
            .with_children(|parent| {
                parent.spawn((Text::new(""), UILabelGameState {}));
            });

        commands
            .spawn((
                Name::new("UI Node Full Screen"),
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                Visibility::Hidden,
                UIRootNode {},
                UINodeFullScreen {},
            ))
            .with_children(|parent| {
                parent.spawn((
                    Text::new("hello\nbevy!"),
                    TextFont {
                        font: asset_server.load("font.ttf"),
                        font_size: 100.0,
                        ..default()
                    },
                    UILabelCenterScreen {},
                ));
            });

        commands
            .spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(65.0),
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                UIRootNode {},
                Name::new("UI"),
            ))
            .with_children(|parent| {
                // Draw hands
                parent
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(350.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        Visibility::Hidden,
                        UIButtonDrawToHand,
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new("draw hand"),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                        ));
                    });

                // Draw hands
                parent
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(350.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        Visibility::Hidden,
                        UIButtonPlayHand,
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new("play hand"),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            // BackgroundColor(Color::srgb(0.9, 0.9, 0.9)),
                        ));
                    });

                parent
                    .spawn((
                        Button,
                        Node {
                            width: Val::Px(350.0),
                            height: Val::Px(65.0),
                            border: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        Visibility::Hidden,
                        UIButtonAdvance,
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new("advance"),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            // BackgroundColor(Color::srgb(0.9, 0.9, 0.9)),
                        ));
                    });
            });
    }
}

pub fn handle_ui_buttons(
    game_state: Res<GameState>,
    mut set: ParamSet<(
        Query<
            (
                &Interaction,
                &mut BackgroundColor,
                &mut BorderColor,
                &Children,
                &UIButtonShuffleDeck,
            ),
            (Changed<Interaction>, With<Button>),
        >,
        Query<
            (
                &Interaction,
                &mut BackgroundColor,
                &mut BorderColor,
                &Children,
                &UIButtonDrawToHand,
            ),
            (Changed<Interaction>, With<Button>),
        >,
        Query<
            (
                &Interaction,
                &mut BackgroundColor,
                &mut BorderColor,
                &Children,
                &UIButtonPlayHand,
            ),
            (Changed<Interaction>, With<Button>),
        >,
        Query<
            (
                &Interaction,
                &mut BackgroundColor,
                &mut BorderColor,
                &Children,
                &UIButtonAdvance,
            ),
            (Changed<Interaction>, With<Button>),
        >,
    )>,
    decks: Query<(Entity, &DeckArea)>,
    mut text_query: Query<&mut Text>,
    mut ew_shuffle: EventWriter<DeckShuffle>,
    mut ew_draw_to_hand: EventWriter<DrawToHand>,
    mut ew_play_hand: EventWriter<EventPlayPokerHand>,
    mut ew_end_card_game: EventWriter<EventEndCardGame>,

    q_decks: Query<(Entity, &DeckArea)>,
) {
    if decks.iter().count() == 0 {
        return;
    }

    let main_deck_entity = q_decks.iter().find(|(_, deck)| deck.marker == 1).unwrap().0;

    for (interaction, mut color, mut border_color, children, _) in &mut set.p0().iter_mut() {
        let mut _text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();

                ew_shuffle.send(DeckShuffle {
                    deck_entity: main_deck_entity,
                });
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }

    for (interaction, mut color, mut border_color, children, _) in &mut set.p1().iter_mut() {
        let mut _text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();

                match game_state.game_type {
                    GameType::Poker => {
                        ew_draw_to_hand.send(DrawToHand {
                            deck_entity: main_deck_entity,
                            num_cards: 1,
                            player: 1,
                        });
                    }
                    GameType::Narrative => {
                        ew_draw_to_hand.send(DrawToHand {
                            deck_entity: main_deck_entity,
                            num_cards: 6,
                            player: 1,
                        });
                    }
                    GameType::CardShop => {}
                }
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }

    for (interaction, mut color, mut border_color, children, _) in &mut set.p2().iter_mut() {
        let mut _text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();
                ew_play_hand.send(EventPlayPokerHand {});
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }

    for (interaction, mut color, mut border_color, children, _) in &mut set.p3().iter_mut() {
        let mut _text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                border_color.0 = RED.into();

                ew_end_card_game.send(EventEndCardGame {});
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
                border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
                border_color.0 = Color::BLACK;
            }
        }
    }
}

pub(crate) fn handle_end_card_game(
    mut commands: Commands,
    q_ui_root_nodes: Query<(Entity, &UIRootNode)>,
    q_cards: Query<(Entity, &Card<VNCard>)>,
    q_play_areas: Query<(Entity, &PlayArea)>,
    q_deck_areas: Query<(Entity, &DeckArea)>,
    mut er_end_game: EventReader<EventEndCardGame>,
    mut ew_switch_next_vn_node: EventWriter<EventSwitchNextNode>,
) {
    for _ in er_end_game.read() {
        for (entity, _) in q_ui_root_nodes.iter() {
            commands.entity(entity).despawn_recursive();
        }

        for (entity, _) in q_cards.iter() {
            commands.entity(entity).despawn_recursive();
        }

        for (entity, _) in q_play_areas.iter() {
            commands.entity(entity).despawn_recursive();
        }

        for (entity, _) in q_deck_areas.iter() {
            commands.entity(entity).despawn_recursive();
        }

        ew_switch_next_vn_node.send(EventSwitchNextNode {});
    }
}

pub(crate) fn handle_buttons_visibility(
    mut game_state: ResMut<GameState>,
    mut ew_update_game_state_ui: EventWriter<EventUpdateGameStateUI>,
) {
    ew_update_game_state_ui.send(EventUpdateGameStateUI {});
}
