mod cards_game;
mod cards_scene;
mod cards_ui;
mod llms;
mod visual_novel;

use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_la_mesa::*;
use bevy_lunex::{prelude::MainUi, UiMinimalPlugins};
use bevy_novel::*;
use bevy_tokio_tasks::*;
use pecs::prelude::*;

use crate::cards_scene::*;
use crate::cards_ui::*;
use crate::llms::*;
use crate::visual_novel::*;

fn main() {
    App::new()
        .add_plugins(())
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            UiMinimalPlugins,
            TokioTasksPlugin::default(),
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
            PecsPlugin,
            NovelPlugin {},
            LaMesaPlugin::<cards_game::VNCard>::default(),
            LLMPlugin {},
        ))
        .add_systems(
            Startup,
            (setup_camera, start_visual_novel, setup_card_scene),
        )
        .add_systems(
            Update,
            (
                handle_new_node,
                handle_llm_response,
                handle_buttons,
                handle_card_press,
                handle_draw_hand,
                handle_deck_rendered_card_game,
                handle_deck_rendered_card_ui,
                handle_hide_ui_overlay,
                handle_play_hand_effect,
                handle_update_game_state_ui,
                handle_play_hand,
                handle_end_game,
            ),
        )
        // Plugin Settings
        .insert_resource(NovelSettings {
            assets_path: "plot".to_string(),
        })
        .insert_resource(LaMesaPluginSettings {
            num_players: 1,
            hand_size: 5,
            back_card_path: "poker-cards/Back_5.png".into(),
        })
        // Events
        .add_event::<EventPlayHand>()
        .add_event::<EventEndGame>()
        .add_event::<EventHideUIOverlay>()
        .add_event::<EventUpdateGameStateUI>()
        .add_event::<EventPlayHandEffect>()
        // Resources
        .insert_resource(GameState {
            max_number_of_draws: 3,
            end_of_game: false,
            enable_play_hand: false,
            ..default()
        })
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera 2d"),
        MainUi,
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 1000.0),
            camera: Camera {
                order: 1,
                ..default()
            },
            ..default()
        },
    ));

    commands.spawn((
        Name::new("Camera 3d"),
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 15.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera {
                order: 2,
                ..default()
            },
            ..default()
        },
    ));
}
