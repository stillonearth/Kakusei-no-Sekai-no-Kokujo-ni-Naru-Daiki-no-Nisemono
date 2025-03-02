use bevy::prelude::*;

use bevy_hui::prelude::*;
use bevy_kira_audio::*;
use bevy_novel::events::EventSwitchNextNode;

use crate::{AppState, EventEndCardGame, EventPlayHand, GameState, GameType};

pub struct GameMenuPlugin;

impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), show_menu)
            .add_systems(OnExit(AppState::Game), despawn_menu)
            .add_systems(Update, (refresh_ui).run_if(in_state(AppState::Game)))
            .add_event::<EventHideMainMenu>()
            .add_event::<EventShowMainMenu>()
            .add_event::<EventRefreshUI>();
    }
}

#[derive(Component)]
pub struct GameMenu {}

#[derive(Event)]
pub struct EventHideMainMenu {}

#[derive(Event)]
pub struct EventShowMainMenu {}

#[derive(Event, PartialEq, Eq)]
pub enum EventRefreshUI {
    PokerMenu(PokerMenuSettings),
    NovelMenu,
    ShopMenu,
    LoadingMenu,
}

#[derive(Event, PartialEq, Eq)]
pub struct PokerMenuSettings {
    pub show_advance_button: bool,
    pub show_score: bool,
    pub score: usize,
}

pub fn show_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut html_funcs: HtmlFunctions,
) {
    // menu
    commands.spawn((
        HtmlNode(asset_server.load("menu/novel_menu.html")),
        TemplateProperties::default()
            .with("advance_button_display", "none")
            .with("score_display", "none")
            .with("score", "")
            .with("title", ""),
        GameMenu {},
    ));

    // game menu button handlers
    html_funcs.register(
        "advance",
        |In(_),
         mut ew_switch_next_node: EventWriter<EventSwitchNextNode>,
         mut ew_play_hand: EventWriter<EventPlayHand>,
         mut ew_end_game: EventWriter<EventEndCardGame>,
         game_state: ResMut<GameState>| {
            if game_state.game_type == GameType::CardShop || game_state.game_type == GameType::Poker
            {
                ew_play_hand.send(EventPlayHand {});
                ew_end_game.send(EventEndCardGame {});
            }
            ew_switch_next_node.send(EventSwitchNextNode {});
        },
    );
}

fn despawn_menu(
    mut commands: Commands,
    q_main_menu_entities: Query<(Entity, &GameMenu)>,
    audio: Res<Audio>,
) {
    for (entity, _) in q_main_menu_entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    audio.stop();
}

fn refresh_ui(
    mut commands: Commands,
    mut er_refresh_ui: EventReader<EventRefreshUI>,
    q_game_menu: Query<(Entity, &GameMenu)>,
    asset_server: Res<AssetServer>,
    game_state: Res<GameState>,
) {
    for event in er_refresh_ui.read() {
        for (entity, _) in q_game_menu.iter() {
            commands.entity(entity).despawn_recursive();
        }

        match event {
            EventRefreshUI::PokerMenu(poker_menu_settings) => {
                let advance_button_display = if poker_menu_settings.show_advance_button {
                    "flex"
                } else {
                    "none"
                };

                let score_display = if poker_menu_settings.show_score {
                    "flex"
                } else {
                    "none"
                };

                commands.spawn((
                    HtmlNode(asset_server.load("menu/novel_menu.html")),
                    TemplateProperties::default()
                        .with("advance_button_display", advance_button_display)
                        .with("score_display", score_display)
                        .with("score", &format!("{}", poker_menu_settings.score))
                        .with("title", "POKER SOLITARE"),
                    GameMenu {},
                ));
            }
            EventRefreshUI::NovelMenu => {
                commands.spawn((
                    HtmlNode(asset_server.load("menu/novel_menu.html")),
                    TemplateProperties::default()
                        .with("advance_button_display", "flex")
                        .with("score_display", "none")
                        .with("score", &format!("{}", game_state.score))
                        .with("title", "CHAPTER 1"),
                    GameMenu {},
                ));
            }
            EventRefreshUI::ShopMenu => {
                commands.spawn((
                    HtmlNode(asset_server.load("menu/novel_menu.html")),
                    TemplateProperties::default()
                        .with("advance_button_display", "flex")
                        .with("score_display", "flex")
                        .with("score", &format!("{}", game_state.score))
                        .with("title", "CARD SHOP"),
                    GameMenu {},
                ));
            }
            EventRefreshUI::LoadingMenu => {
                commands.spawn((
                    HtmlNode(asset_server.load("menu/novel_menu.html")),
                    TemplateProperties::default()
                        .with("advance_button_display", "none")
                        .with("score_display", "none")
                        .with("score", &format!("{}", game_state.score))
                        .with("title", "GENERATING CHAPTER"),
                    GameMenu {},
                ));
            }
        }
    }
}
