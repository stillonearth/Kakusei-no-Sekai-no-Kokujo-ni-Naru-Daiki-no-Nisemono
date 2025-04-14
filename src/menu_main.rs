use std::time::Duration;

use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};
use bevy_asset_loader::prelude::*;
use bevy_defer::AsyncCommandsExtension;
use bevy_defer::AsyncWorld;
use bevy_hui::prelude::*;
use bevy_kira_audio::*;
use bevy_la_mesa::{
    events::{DeckRendered, DeckShuffle, DrawToTable, RenderDeck},
    Card, CardOnTable, DeckArea, PlayArea,
};
use rand::Rng;

use crate::{
    cards_game::{filter_narrative_cards, VNCard},
    AppState, GameState,
};

pub struct MainMenuPlugin;

const SHADER_ASSET_PATH: &str = "shaders/balatro.wgsl";

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::Loading1)
                .continue_to_state(AppState::Loading2)
                .load_collection::<MainMenuAssets>(),
        )
        .add_event::<StartCardAnimation>()
        .add_plugins(MaterialPlugin::<CustomMaterial>::default())
        .add_systems(
            Update,
            (shuffle_deck, handle_start_card_animation, animate_card)
                .run_if(in_state(AppState::MainMenu)),
        )
        .add_systems(OnEnter(AppState::MainMenu), show_menu)
        .add_systems(OnExit(AppState::MainMenu), despawn_menu);
    }
}

#[derive(Component)]
pub struct MainMenuResource {}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {}

#[derive(Component)]
pub struct AnimatedCard {
    pub animation_start: f32,
    pub animation_speed: f32,
}

#[derive(Event)]
pub struct StartCardAnimation {}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}

#[derive(AssetCollection, Resource)]
pub struct MainMenuAssets {
    #[asset(path = "music/balatro_theme.ogg")]
    balatro_theme: Handle<bevy_kira_audio::AudioSource>,
}

pub fn show_menu(
    main_menu_assets: Res<MainMenuAssets>,
    audio: Res<Audio>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut html_funcs: HtmlFunctions,
    game_state: Res<GameState>,
    mut ew_render_deck: EventWriter<RenderDeck<VNCard>>,
) {
    // table
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default())),
        MeshMaterial3d(custom_materials.add(CustomMaterial {})),
        Transform::from_translation(Vec3::new(0.0, -1.0, 0.0)).with_scale(Vec3::ONE * 50.0),
        ZIndex(2),
        MainMenuResource {},
    ));

    // menu
    commands.spawn((
        HtmlNode(asset_server.load("menu/main_menu.html")),
        TemplateProperties::default().with("user_wallet_address", &game_state.wallet.address),
        MainMenuResource {},
    ));

    audio
        .play(main_menu_assets.balatro_theme.clone())
        .loop_from(0.5)
        .fade_in(AudioTween::new(
            Duration::from_secs(2),
            AudioEasing::OutPowi(2),
        ));

    // main menu handler
    html_funcs.register(
        "start_game",
        |In(_), mut app_state: ResMut<NextState<AppState>>| {
            app_state.set(AppState::Game);
            // ew_start_game.send(EventStartGame {});
        },
    );

    // deck
    let deck_shop_cards = commands
        .spawn((
            Mesh3d(meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10))),
            MeshMaterial3d(standard_materials.add(Color::BLACK)),
            Transform::from_translation(Vec3::new(0.0, 2.1, 0.5))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI / 2.0)),
            Visibility::Hidden,
            DeckArea { marker: 1 },
            Name::new("Deck 1 -- Shop Cards"),
            MainMenuResource {},
        ))
        .id();

    // Play Area
    for i in 0..8 {
        for j in 0..8 {
            let material = MeshMaterial3d(standard_materials.add(Color::srgb_u8(124, 144, 255)));

            commands.spawn((
                Mesh3d(meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10))),
                material,
                Transform::from_translation(Vec3::new(
                    -13.0 + 3.7 * (i as f32),
                    3.0,
                    15.0 - 4.2 * (j as f32),
                )),
                Visibility::Hidden,
                PlayArea {
                    marker: i * 8 + j,
                    player: 1,
                },
                Name::new(format!("Play Area {} {}", i, j)),
                RayCastPickable,
                MainMenuResource {},
            ));
        }
    }

    ew_render_deck.send(RenderDeck::<VNCard> {
        deck_entity: deck_shop_cards,
        deck: filter_narrative_cards(game_state.game_deck.clone()).unwrap(),
    });
}

pub fn despawn_menu(
    mut commands: Commands,
    q_main_menu_entities: Query<(Entity, &MainMenuResource)>,
    audio: Res<Audio>,
) {
    for (entity, _) in q_main_menu_entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    audio.stop();
}

pub fn handle_start_card_animation(
    mut commands: Commands,
    mut er_start_card_animation: EventReader<StartCardAnimation>,
    q_cards_on_table: Query<(Entity, &CardOnTable)>,
    time: Res<Time>,
) {
    for _ in er_start_card_animation.read() {
        let mut rng = rand::thread_rng();
        for (entity, _) in q_cards_on_table.iter() {
            let random_number: f32 = rng.gen_range(30.0..120.0);
            commands.entity(entity).insert(AnimatedCard {
                animation_start: time.elapsed_secs(),
                animation_speed: random_number,
            });
        }
    }
}

pub fn animate_card(
    mut q_cards_on_table: Query<(Entity, &mut Transform, &AnimatedCard)>,
    time: Res<Time>,
) {
    for (i, (_, mut transform, ac)) in q_cards_on_table.iter_mut().enumerate() {
        if i % 3 == 0 {
            transform.rotate_local_x(
                (time.elapsed_secs() - ac.animation_start).cos() / ac.animation_speed,
            );
            transform.rotate_local_y(
                (time.elapsed_secs() - ac.animation_start).sin() / ac.animation_speed,
            );
        }
        if i % 3 != 0 {
            transform.rotate_local_x(
                (time.elapsed_secs() - ac.animation_start).sin() / ac.animation_speed,
            );
            transform.rotate_local_y(
                (time.elapsed_secs() - ac.animation_start).cos() / ac.animation_speed,
            );
        }
    }
}

pub fn shuffle_deck(
    mut commands: Commands,
    mut er_deck_rendered: EventReader<DeckRendered>,
    mut ew_shuffle: EventWriter<DeckShuffle>,
    q_decks: Query<(Entity, &DeckArea)>,
    q_cards: Query<(Entity, &Card<VNCard>)>,
) {
    let deck_idle_time = 1.0;
    let main_deck_entity = q_decks.iter().find(|(_, deck)| deck.marker == 1).unwrap().0;
    for _ in er_deck_rendered.read() {
        ew_shuffle.send(DeckShuffle {
            deck_entity: main_deck_entity,
            duration: 20,
        });

        for (entity, _) in q_cards.iter() {
            commands.entity(entity).insert(MainMenuResource {});
        }

        let main_deck_entity = q_decks.iter().find(|(_, deck)| deck.marker == 1).unwrap().0;
        let n_cards_on_table = q_cards.iter().count();
        let shuffle_animation_time = ((n_cards_on_table * 10) as f32) * 0.001;

        commands.spawn_task(move || async move {
            AsyncWorld.sleep(deck_idle_time).await;
            AsyncWorld.sleep(shuffle_animation_time).await;

            let play_area_markers: Vec<usize> = (0..64).collect();
            AsyncWorld.send_event(DrawToTable {
                deck_entity: main_deck_entity,
                play_area_markers,
                player: 1,
                duration: 30,
            })?;

            AsyncWorld.sleep(10).await;
            AsyncWorld.send_event(StartCardAnimation {})?;

            Ok(())
        });
    }
}
