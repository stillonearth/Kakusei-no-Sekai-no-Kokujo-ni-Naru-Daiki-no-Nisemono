mod cards;
mod llms;
mod visual_novel;

use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_la_mesa::*;
use bevy_lunex::{prelude::MainUi, UiMinimalPlugins};
use bevy_novel::*;
use bevy_tokio_tasks::*;
use cards::{Chip, PokerCard};
use llms::LLMPlugin;

use crate::cards::*;
use crate::visual_novel::*;

fn main() {
    App::new()
        .add_plugins(())
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            UiMinimalPlugins,
            TokioTasksPlugin::default(),
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
            NovelPlugin {},
            LaMesaPlugin::<PokerCard, Chip>::default(),
            LLMPlugin {},
        ))
        .add_systems(
            Startup,
            (
                setup_camera,
                // VN setting
                start_visual_novel,
                // cards ui
                setup_card_scene,
                setup_ui,
            ),
        )
        .add_systems(
            Update,
            (
                handle_new_node,
                handle_llm_response,
                handle_buttons,
                handle_card_press,
                handle_play_hand,
            ),
        )
        // Plugin Settings
        .insert_resource(NovelSettings {
            assets_path: "plot".to_string(),
        })
        .insert_resource(LaMesaPluginSettings {
            num_players: 1,
            hand_size: 5,
            back_card_path: "cards/Back_5.png".into(),
        })
        // Events
        .add_event::<PlayHand>()
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
