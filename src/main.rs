#![feature(let_chains)]

mod cards_game;
mod cards_scene;
mod cards_solitaire;
mod cards_ui;
mod llm;
mod text2img;
mod visual_novel;

use bevy::asset::AssetMetaCheck;
use bevy::color::palettes::css::WHITE;
use bevy_wasm_tasks::*;

use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_defer::AsyncPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_la_mesa::*;
use bevy_novel::*;
use cards_game::NarrativeCards;
use rpy_asset_loader::Rpy;
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
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        canvas: Some("#bevy".to_string()),
                        ..default()
                    }),
                    ..default()
                }),
            AsyncPlugin::default_settings(),
            JsonAssetPlugin::<NarrativeCards>::new(&["json"]),
            LaMesaPlugin::<cards_game::VNCard>::default(),
            LLMPlugin {},
            MeshPickingPlugin,
            NovelPlugin {},
            TasksPlugin::default(),
            Text2ImagePlugin {},
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        ))
        .add_systems(Startup, (setup_camera_and_light, load_resources))
        .add_systems(
            Update,
            (start_visual_novel,).run_if(in_state(AppState::Loading)),
        )
        .add_systems(
            Update,
            ((
                handle_card_position_hover,
                handle_card_position_out,
                handle_card_position_press,
                handle_card_press_cardplay,
                handle_deck_rendered_card_game,
                handle_deck_rendered_card_ui,
                handle_draw_to_hand,
                handle_draw_to_table,
                handle_end_card_game,
                handle_hide_ui_overlay,
                handle_llm_response,
                handle_new_vn_node,
                handle_play_hand_effect,
                handle_play_hand,
                handle_text_2_image_response,
                handle_ui_buttons,
            )
                .chain())
            .run_if(in_state(AppState::Novel)),
        )
        .add_systems(
            Update,
            ((
                handle_start_card_shop,
                apply_deferred,
                handle_start_narrative_game,
                apply_deferred,
                handle_start_poker_game,
                // apply_deferred,
            )
                .chain())
            .run_if(in_state(AppState::Novel)),
        )
        .add_systems(
            Update,
            (
                handle_ui_update_game_state,
                handle_card_on_table_hover.after(handle_card_on_table_out),
                handle_card_on_table_out,
                handle_card_press_cardshop,
            )
                .run_if(in_state(AppState::Novel)),
        )
        // Plugin Settings
        .insert_resource(NovelSettings {
            assets_path: "plot".to_string(),
            pause_handle_switch_node: false,
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
            ui_show_advance_button: false,
            ui_enable_play_hand: false,
            score: 0,
            collected_deck: vec![],
            ..default()
        })
        .insert_resource(MeshPickingSettings {
            require_markers: true,
            ray_cast_visibility: RayCastVisibility::Any,
        })
        .init_state::<AppState>()
        .run();
}

fn setup_camera_and_light(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera 2d"),
        Camera2d,
        Camera {
            order: 1,
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1000.0),
    ));

    commands.spawn((
        Name::new("Camera 3d"),
        Camera3d::default(),
        Camera {
            order: 2,
            ..default()
        },
        RayCastPickable,
        Transform::from_xyz(0.0, 18.2, 11.9).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.insert_resource(AmbientLight {
        color: WHITE.into(),
        brightness: 1000.0,
    });
}

// Initialization

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Loading,
    Novel,
}

#[derive(Resource, Deref, DerefMut)]
struct ScenarioHandle(Handle<Rpy>);

#[derive(Resource, Deref, DerefMut)]
struct NarrativeCardsHandle(Handle<NarrativeCards>);

fn load_resources(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scenario_handle = ScenarioHandle(asset_server.load("plot/intro.rpy"));
    commands.insert_resource(scenario_handle);

    let narrative_cards_handle =
        NarrativeCardsHandle(asset_server.load("narrative-cards/cards.json"));
    commands.insert_resource(narrative_cards_handle);
}
