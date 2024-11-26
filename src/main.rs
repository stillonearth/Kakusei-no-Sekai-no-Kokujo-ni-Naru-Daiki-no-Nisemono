mod cards_game;
mod cards_scene;
mod cards_solitaire;
mod cards_ui;
mod llm;
mod text2img;
mod visual_novel;

use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_la_mesa::*;
use bevy_lunex::{prelude::MainUi, UiMinimalPlugins};
use bevy_novel::*;
use bevy_tokio_tasks::*;
use pecs::prelude::*;
use text2img::Text2ImagePlugin;

use crate::cards_scene::*;
use crate::cards_solitaire::*;
use crate::cards_ui::*;
use crate::llm::*;
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
            Text2ImagePlugin {},
        ))
        .add_systems(
            Startup,
            (setup_camera, start_visual_novel, setup_card_scene),
        )
        .add_systems(
            Update,
            (
                handle_card_position_hover,
                handle_card_position_out,
                handle_card_position_press,
                handle_card_press,
                handle_deck_rendered_card_game,
                handle_deck_rendered_card_ui,
                handle_draw_hand,
                handle_end_card_game,
                handle_hide_ui_overlay,
                handle_llm_response,
                handle_new_vn_node,
                handle_play_hand_effect,
                handle_play_hand,
                handle_start_card_shop,
                handle_start_narrative_game,
                handle_start_poker_game,
                handle_text_2_image_response,
                handle_ui_buttons,
                handle_ui_update_game_state,
            ),
        )
        // Plugin Settings
        .insert_resource(NovelSettings {
            assets_path: "plot".to_string(),
        })
        // Events
        .add_event::<EventCardPositionHover>()
        .add_event::<EventCardPositionOut>()
        .add_event::<EventCardPositionPress>()
        .add_event::<EventEndCardGame>()
        .add_event::<EventHideUIOverlay>()
        .add_event::<EventPlayPokerHand>()
        .add_event::<EventPlayPokerHandEffect>()
        .add_event::<EventStartNarrativeGame>()
        .add_event::<EventStartPokerGame>()
        .add_event::<EventStartNarrativeCardShop>()
        .add_event::<EventUpdateGameStateUI>()
        // Resources
        .insert_resource(GameState {
            max_n_poker_draws: 25,
            ui_end_of_game: false,
            ui_enable_play_hand: false,
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
            transform: Transform::from_xyz(0.0, 18.2, 11.9).looking_at(Vec3::ZERO, Vec3::Y),
            camera: Camera {
                order: 2,
                ..default()
            },
            ..default()
        },
    ));
}
