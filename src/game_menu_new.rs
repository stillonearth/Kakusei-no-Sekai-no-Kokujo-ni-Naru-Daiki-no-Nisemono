use bevy::prelude::*;

use bevy_hui::prelude::*;
use bevy_kira_audio::*;
use bevy_novel::events::EventSwitchNextNode;

use crate::AppState;

pub struct GameMenuPlugin;

impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Novel), show_menu)
            .add_systems(OnExit(AppState::Novel), despawn_menu);
    }
}

#[derive(Component)]
pub struct GameMenuComponent {}

pub fn show_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut html_funcs: HtmlFunctions,
) {
    // menu
    commands.spawn((
        HtmlNode(asset_server.load("menu/game_menu.html")),
        TemplateProperties::default(),
        GameMenuComponent {},
    ));

    // game menu button handlers
    html_funcs.register(
        "advance",
        |In(_), mut ew_switch_next_node: EventWriter<EventSwitchNextNode>| {
            ew_switch_next_node.send(EventSwitchNextNode {});
        },
    );
}

pub fn despawn_menu(
    mut commands: Commands,
    q_main_menu_entities: Query<(Entity, &GameMenuComponent)>,
    audio: Res<Audio>,
) {
    for (entity, _) in q_main_menu_entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    audio.stop();
}
