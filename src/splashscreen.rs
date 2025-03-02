use crate::AppState;
use bevy::prelude::*;

#[derive(Component)]
struct Splashscreen;

pub struct SplashscreenPlugin;

impl Plugin for SplashscreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnExit(AppState::Loading2),
            |mut commands: Commands, q_menu_components: Query<(Entity, &Splashscreen)>| {
                for (e, _) in q_menu_components.iter() {
                    commands.entity(e).despawn_recursive();
                }
            },
        )
        .add_systems(OnEnter(AppState::Loading1), setup_splashscreen);
    }
}

fn setup_splashscreen(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                // width: Val::Percent(100.0),
                // height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            Splashscreen,
            ZIndex(0),
            Name::new("cutscene image container"),
        ))
        .with_children(|parent| {
            parent.spawn((
                Sprite::from_image(asset_server.load("splash.png")),
                Transform::from_scale(Vec3::ONE * 1.0),
            ));
        });
}
