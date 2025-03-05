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
            .add_systems(
                Update,
                (render_ui, refresh_ui).run_if(in_state(AppState::Game)),
            )
            .add_event::<EventHideMainMenu>()
            .add_event::<EventShowMainMenu>()
            .add_event::<EventRefreshUI>()
            .add_event::<EventRenderUI>();
    }
}

#[derive(Component)]
pub struct GameMenu {}

#[derive(Event)]
pub struct EventHideMainMenu {}

#[derive(Event)]
pub struct EventShowMainMenu {}

/// Update Menu disaply variables
#[derive(Event, PartialEq, Eq)]
pub enum EventRefreshUI {
    PokerMenu(PokerMenuSettings),
    NovelMenu,
    ShopMenu,
    LoadingMenu,
    Narrative,
}

/// Despawn previous menu template and render a new one
#[derive(Event, PartialEq, Eq)]
pub enum EventRenderUI {
    PokerMenu(PokerMenuSettings),
    NovelMenu,
    ShopMenu,
    LoadingMenu,
    Narrative,
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
            if game_state.game_type == GameType::CardShop
                || game_state.game_type == GameType::Poker
                || game_state.game_type == GameType::Narrative
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

fn render_ui(
    mut commands: Commands,
    mut er_refresh_ui: EventReader<EventRenderUI>,
    q_game_menu: Query<(Entity, &GameMenu)>,
    asset_server: Res<AssetServer>,
    game_state: Res<GameState>,
) {
    for event in er_refresh_ui.read() {
        for (entity, _) in q_game_menu.iter() {
            commands.entity(entity).despawn_recursive();
        }

        match event {
            EventRenderUI::PokerMenu(_) => {
                commands.spawn((
                    HtmlNode(asset_server.load("menu/poker_menu.html")),
                    TemplateProperties::default(),
                    GameMenu {},
                    Name::new("poker menu"),
                ));
            }
            EventRenderUI::NovelMenu => {
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
            EventRenderUI::ShopMenu => {
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
            EventRenderUI::LoadingMenu => {
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
            EventRenderUI::Narrative => {
                commands.spawn((
                    HtmlNode(asset_server.load("menu/novel_menu.html")),
                    TemplateProperties::default()
                        .with("advance_button_display", "none")
                        .with("score_display", "none")
                        .with("score", &format!("{}", game_state.score))
                        .with("title", "Narrative"),
                    GameMenu {},
                ));
            }
        }
    }
}

fn refresh_ui(
    // mut commands: Commands,
    mut er_refresh_ui: EventReader<EventRefreshUI>,
    mut q_text_labels: Query<(Entity, &mut Text, &Tags)>,
    // game_state: Res<GameState>,
) {
    for event in er_refresh_ui.read() {
        match event {
            EventRefreshUI::PokerMenu(poker_menu_settings) => {
                for (_, mut text, tags) in q_text_labels.iter_mut() {
                    if let Some(marker) = tags.get("marker")
                        && marker == "text_score"
                    {
                        *text = Text::new(format!("${}", poker_menu_settings.score));
                    }
                }
            }
            _ => {}
        }
    }
}
