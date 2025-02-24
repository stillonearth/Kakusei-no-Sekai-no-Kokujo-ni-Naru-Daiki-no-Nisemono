use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};

use crate::AppState;

pub struct MainMenuPlugin;

const SHADER_ASSET_PATH: &str = "shaders/balatro.wgsl";

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<CustomMaterial>::default())
            .add_systems(Startup, render_menu)
            .add_systems(Update, (show_menu,).run_if(in_state(AppState::Loading)));
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}

pub fn render_menu(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    // cube
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default())),
        MeshMaterial3d(materials.add(CustomMaterial {})),
        Transform::default()
            .with_scale(Vec3::ONE * 35.0)
            .with_rotation(Quat::from_rotation_x(0.7)),
    ));
}

pub fn show_menu(mut app_state: ResMut<NextState<AppState>>) {
    println!("here");
    app_state.set(AppState::MainMenu);
}
