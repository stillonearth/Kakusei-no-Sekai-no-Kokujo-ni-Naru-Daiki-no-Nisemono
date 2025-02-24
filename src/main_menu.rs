use std::time::Duration;

use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use bevy_asset_loader::prelude::*;
use bevy_hui::{prelude::*, HuiPlugin};
use bevy_kira_audio::*;

use crate::AppState;

pub struct MainMenuPlugin;

const SHADER_ASSET_PATH: &str = "shaders/balatro.wgsl";

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::Loading)
                .continue_to_state(AppState::MainMenu)
                .load_collection::<MainMenuAssets>(),
        )
        .add_plugins((MaterialPlugin::<CustomMaterial>::default(), HuiPlugin))
        // .add_systems(Startup, render_menu)
        .add_systems(OnEnter(AppState::MainMenu), show_menu);
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}

#[derive(AssetCollection, Resource)]
struct MainMenuAssets {
    #[asset(path = "music/balatro_theme.ogg")]
    balatro_theme: Handle<bevy_kira_audio::AudioSource>,
}

pub fn show_menu(
    // mut app_state: ResMut<NextState<AppState>>,
    main_menu_assets: Res<MainMenuAssets>,
    audio: Res<Audio>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // background
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default())),
        MeshMaterial3d(materials.add(CustomMaterial {})),
        Transform::default()
            .with_scale(Vec3::ONE * 35.0)
            .with_rotation(Quat::from_rotation_x(0.7)),
        ZIndex(2),
    ));

    // menu
    commands.spawn((
        HtmlNode(asset_server.load("menu/menu.html")),
        TemplateProperties::default(), //.with("title", "Test-title"),
    ));

    audio
        .play(main_menu_assets.balatro_theme.clone())
        .loop_from(0.5)
        .fade_in(AudioTween::new(
            Duration::from_secs(2),
            AudioEasing::OutPowi(2),
        ));
}
