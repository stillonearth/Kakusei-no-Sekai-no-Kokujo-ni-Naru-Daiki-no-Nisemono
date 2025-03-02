use bevy::prelude::*;

use bevy_hui::prelude::*;
use bevy_kira_audio::*;
use bevy_novel::events::EventSwitchNextNode;

use crate::AppState;

pub struct GameMenuPlugin;

impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), show_menu)
            .add_systems(OnExit(AppState::Game), despawn_menu)
            .add_systems(
                Update,
                (handle_hide_game_menu, handle_show_game_menu, refresh_ui)
                    .run_if(in_state(AppState::Game)),
            )
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
}

#[derive(Event, PartialEq, Eq)]
pub struct PokerMenuSettings {
    pub show_advance_button: bool,
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
        TemplateProperties::default(),
        GameMenu {},
    ));

    // game menu button handlers
    html_funcs.register(
        "advance",
        |In(_), mut ew_switch_next_node: EventWriter<EventSwitchNextNode>| {
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

fn handle_hide_game_menu(
    mut er_hide_main_menu: EventReader<EventHideMainMenu>,
    mut q_main_menu: Query<(Entity, &mut Visibility, &GameMenu)>,
) {
    for _ in er_hide_main_menu.read() {
        for (_, mut visibility, _) in q_main_menu.iter_mut() {
            *visibility = Visibility::Hidden;
        }
    }
}

fn handle_show_game_menu(
    mut er_show_main_menu: EventReader<EventShowMainMenu>,
    mut q_main_menu: Query<(Entity, &mut Visibility, &GameMenu)>,
) {
    for _ in er_show_main_menu.read() {
        for (_, mut visibility, _) in q_main_menu.iter_mut() {
            *visibility = Visibility::Hidden;
        }
    }
}

fn refresh_ui(
    mut commands: Commands,
    mut er_refresh_ui: EventReader<EventRefreshUI>,
    q_main_menu_entities: Query<(Entity, &GameMenu)>,
    asset_server: Res<AssetServer>,
) {
    for event in er_refresh_ui.read() {
        // despawn old menu
        for (entity, _) in q_main_menu_entities.iter() {
            commands.entity(entity).despawn_recursive();
        }

        // spawn a new one
        match event {
            EventRefreshUI::PokerMenu(poker_menu_settings) => {
                let advance_button_display = if poker_menu_settings.show_advance_button {
                    "flex"
                } else {
                    "none"
                };

                commands.spawn((
                    HtmlNode(asset_server.load("menu/game_menu.html")),
                    TemplateProperties::default()
                        .with("advance_button_display", advance_button_display)
                        .with("score", &format!("{}", poker_menu_settings.score)),
                    GameMenu {},
                ));
            }
            EventRefreshUI::NovelMenu => todo!(),
            EventRefreshUI::ShopMenu => todo!(),
        }
    }
}
