mod cards;
mod llms;

use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_la_mesa::{
    events::{DrawHand, RenderDeck},
    DeckArea, HandArea, LaMesaPlugin, LaMesaPluginSettings,
};
use bevy_lunex::{prelude::MainUi, UiMinimalPlugins};
use bevy_novel::{
    events::{EventHandleNode, EventStartScenario, EventSwitchNextNode},
    NovelData, NovelPlugin, NovelSettings,
};
use bevy_tokio_tasks::*;
use cards::{Chip, PokerCard};
use llms::{EventLLMRequest, EventLLMResponse, LLMPlugin};
use renpy_parser::{
    group_logical_lines,
    lexer::Lexer,
    list_logical_lines,
    parsers::{parse_block, AST},
};

use crate::cards::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, UiMinimalPlugins))
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
        )
        .add_plugins((
            NovelPlugin {},
            LaMesaPlugin::<PokerCard, Chip>::default(),
            LLMPlugin {},
            TokioTasksPlugin::default(),
        ))
        .insert_resource(NovelSettings {
            assets_path: "plot".to_string(),
        })
        .insert_resource(LaMesaPluginSettings {
            num_players: 1,
            hand_size: 7,
            back_card_path: "cards/card-back2.png".into(),
        })
        .add_systems(
            Startup,
            (setup_camera, start_visual_novel, setup_card_scene),
        )
        .add_systems(Update, (handle_new_node, handle_llm_response))
        .run();
}

fn setup_card_scene(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Deck
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(2.5, 3.5).subdivisions(10)),
            material: materials.add(Color::BLACK),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))
                .with_rotation(Quat::from_rotation_y(std::f32::consts::PI / 2.0)),
            visibility: Visibility::Hidden,
            ..default()
        },
        DeckArea { marker: 1 },
        Name::new("Deck 1 -- Play Cards"),
    ));

    // Hand
    commands.spawn((
        Name::new("HandArea - Player 1"),
        TransformBundle {
            local: Transform::from_translation(Vec3::new(0.0, 1.5, 5.8))
                .with_rotation(Quat::from_rotation_x(std::f32::consts::PI / 4.0)),
            ..default()
        },
        HandArea { player: 1 },
    ));
}

fn load_scenario(path: String) -> Vec<AST> {
    let lines = list_logical_lines(&path).unwrap();
    let blocks = group_logical_lines(lines).unwrap();

    let l = &mut Lexer::new(blocks.clone(), true);

    let (ast, _) = parse_block(l);
    ast
}

fn start_visual_novel(
    mut ew_start_scenario: EventWriter<EventStartScenario>,
    mut ew_render_deck: EventWriter<RenderDeck<PokerCard>>,
) {
    let path = "assets/plot/intro.rpy";
    let ast = load_scenario(path.to_string());

    ew_start_scenario.send(EventStartScenario { ast });
    ew_render_deck.send(RenderDeck::<PokerCard> {
        marker: 1,
        deck: load_poker_deck(),
    });
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

fn handle_llm_response(
    mut novel_data: ResMut<NovelData>,
    mut er_llm_response: EventReader<EventLLMResponse>,
    mut ew_switch_next_node: EventWriter<EventSwitchNextNode>,
) {
    for event in er_llm_response.read() {
        novel_data.push_text_node(None, event.0.clone());
        ew_switch_next_node.send(EventSwitchNextNode {});
    }
}

fn handle_new_node(
    mut novel_data: ResMut<NovelData>,
    mut ew_switch_next_node: EventWriter<EventSwitchNextNode>,
    mut er_handle_node: EventReader<EventHandleNode>,
    mut ew_llm_request: EventWriter<EventLLMRequest>,
    mut ew_draw: EventWriter<DrawHand>,
) {
    for event in er_handle_node.read() {
        match event.ast.clone() {
            AST::Return(_, _) => {
                ew_draw.send(DrawHand {
                    deck_marker: 1,
                    num_cards: 5,
                    player: 1,
                });

                // when sending llm request indicate user that
                novel_data.push_text_node(
                    Some("AI".to_string()),
                    "I'm meditating the answer".to_string(),
                );
                ew_switch_next_node.send(EventSwitchNextNode {});

                ew_llm_request.send(EventLLMRequest(
                    "Say something funny in one simple sentence.".to_string(),
                ));
            }
            _ => {}
        }
    }
}
