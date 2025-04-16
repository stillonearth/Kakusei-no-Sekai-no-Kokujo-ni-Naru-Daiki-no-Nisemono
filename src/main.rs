#![feature(let_chains)]

mod api_llm;
mod api_nft;
mod api_text2img;
mod cards_game;
mod cards_scene;
mod cards_solitaire;
mod menu_game;
mod menu_main;
mod splashscreen;
mod visual_novel;
mod wasm;

use api_nft::EventLoadNFTRequest;
use bevy::asset::AssetMetaCheck;
use bevy::color::palettes::css::WHITE;
use bevy_hui::HuiPlugin;
use bevy_kira_audio::AudioPlugin;
use bevy_wasm_tasks::*;

use bevy::{input::common_conditions::input_toggle_active, prelude::*};
use bevy_common_assets::json::JsonAssetPlugin;
use bevy_defer::AsyncPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_la_mesa::*;
use bevy_modern_pixel_camera::prelude::*;
use bevy_novel::*;

use api_nft::NFTPlugin;
use api_text2img::Text2ImagePlugin;
use cards_game::CharacterCards;
use cards_game::NarrativeCards;
use cards_game::PokerCombination;
use cards_game::PsychosisCards;
use cards_game::VNCard;
use cards_game::VNCardMetadata;
use menu_game::EventRenderUI;
use rpy_asset_loader::Rpy;
use splashscreen::SplashscreenPlugin;

use crate::api_llm::*;
use crate::cards_scene::*;
use crate::cards_solitaire::*;
use crate::menu_game::GameMenuPlugin;
use crate::menu_main::*;
use crate::visual_novel::*;

pub const API_ENDPOINT: &str = "https://kakuseinosekainokokujoninarudaikinonisemono.space/api";

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (1150., 720.).into(),
                        canvas: Some("#bevy".to_string()),
                        ..default()
                    }),
                    ..default()
                }),
            AsyncPlugin::default_settings(),
            JsonAssetPlugin::<NarrativeCards>::new(&["json"]),
            JsonAssetPlugin::<CharacterCards>::new(&["json"]),
            JsonAssetPlugin::<PsychosisCards>::new(&["json"]),
            LaMesaPlugin::<cards_game::VNCard>::default(),
            MeshPickingPlugin,
            NovelPlugin {},
            TasksPlugin::default(),
            AudioPlugin,
            HuiPlugin,
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::Escape)),
            PixelCameraPlugin,
        ))
        .add_plugins((
            SplashscreenPlugin,
            LLMPlugin,
            NFTPlugin,
            Text2ImagePlugin,
            MainMenuPlugin,
            GameMenuPlugin,
        ))
        .add_systems(Startup, (setup_camera_and_light, load_resources))
        .add_systems(Update, (load_cards,).run_if(in_state(AppState::Loading2)))
        .add_systems(OnEnter(AppState::Game), start_visual_novel)
        .add_systems(
            Update,
            ((
                handle_card_position_hover,
                handle_card_position_out,
                handle_card_position_press,
                handle_card_press_cardplay,
                handle_deck_rendered,
                handle_draw_to_hand,
                handle_draw_to_table,
                handle_llm_response,
                handle_new_vn_node,
                handle_event_game_over,
                handle_play_hand,
                handle_text_2_image_response,
                handle_download_image_response,
                handle_end_card_game,
            )
                .chain())
            .run_if(in_state(AppState::Game)),
        )
        .add_systems(
            Update,
            ((handle_new_vn_node, handle_download_image_response).chain())
                .run_if(in_state(AppState::NovelPlayer)),
        )
        .add_systems(
            Update,
            ((
                handle_start_card_shop,
                apply_deferred,
                handle_start_narrative_game,
                apply_deferred,
                handle_start_poker_game,
                apply_deferred,
                poker_handle_place_card_on_table
                    .after(bevy_la_mesa::events::handle_place_card_on_table::<cards_game::VNCard>),
            )
                .chain())
            .run_if(in_state(AppState::Game)),
        )
        .add_systems(
            Update,
            ((start_visual_novel,).chain()).run_if(in_state(AppState::MainMenu)),
        )
        .add_systems(
            Update,
            (
                handle_card_on_table_hover,
                handle_card_on_table_out,
                cardshop_handle_card_press,
            )
                .run_if(in_state(AppState::Game)),
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
        .add_event::<EventPlayHand>()
        .add_event::<EventStartNarrativeGame>()
        .add_event::<EventStartPokerGame>()
        .add_event::<EventStartNarrativeCardShop>()
        .add_event::<EventGameOver>()
        // Resources
        .insert_resource(GameState {
            max_n_poker_draws: 25,
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

// ------
// States
// ------

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    Loading1,
    Loading2,
    Loading3,
    Game,
    NovelPlayer,
    MainMenu,
}

// ---------
// Resources
// ---------

#[derive(Resource, Default, Debug, PartialEq, Eq)]
pub(crate) enum GameType {
    #[default]
    VisualNovel,
    Poker,
    Narrative,
    CardShop,
    VisualNovelPlayer,
}

#[derive(Default)]
pub(crate) struct CryptoWallet {
    pub address: String,
}

#[derive(Resource, Default)]
pub(crate) struct GameState {
    pub game_deck: Vec<VNCard>,
    pub collected_deck: Vec<VNCard>,
    pub game_type: GameType,
    pub max_n_poker_draws: usize,
    pub n_draws: usize,
    pub n_turns: usize,
    pub n_vn_node_scene_request: usize,
    pub n_vn_node: usize,
    pub narrative_conflicts: Vec<String>,
    pub narrative_plot_twists: Vec<String>,
    pub narrative_settings: Vec<String>,
    pub characters: Vec<String>,
    pub psychosis: Vec<String>,
    pub narrative_story_so_far: Vec<String>,
    pub poker_combinations: Vec<PokerCombination>,
    pub score: isize,
    pub current_menu_type: EventRenderUI,
    pub wallet: CryptoWallet,
    pub player_nft_url: Option<String>,
}

#[derive(Resource, Deref, DerefMut)]
struct ScenarioHandle(Handle<Rpy>);

#[derive(Resource, Deref, DerefMut)]
struct NarrativeCardsHandle(Handle<NarrativeCards>);

#[derive(Resource, Deref, DerefMut)]
struct CharacterCardsHandle(Handle<CharacterCards>);

#[derive(Resource, Deref, DerefMut)]
struct PsychosisCardsHandle(Handle<PsychosisCards>);

fn load_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game_state: ResMut<GameState>,
) {
    let scenario_handle = ScenarioHandle(asset_server.load("plot/intro.rpy"));
    commands.insert_resource(scenario_handle);

    let character_cards_handle =
        CharacterCardsHandle(asset_server.load("character-cards/cards.json"));
    commands.insert_resource(character_cards_handle);

    let narrative_cards_handle =
        NarrativeCardsHandle(asset_server.load("narrative-cards/cards.json"));
    commands.insert_resource(narrative_cards_handle);

    let psychosis_cards_handle =
        PsychosisCardsHandle(asset_server.load("psychosis-cards/cards.json"));
    commands.insert_resource(psychosis_cards_handle);

    game_state.wallet.address = "0x971C6CDa7EDE9db62732D896995c9ee3A3196e40".to_string();
    // game_state.game_type = GameType::VisualNovelPlayer;
    // game_state.player_nft_url =
    //     Some("https://kakuseinosekainokokujoninarudaikinonisemono.space/api/nft/34".to_string());

    // load app settings from wasm container
    #[cfg(target_arch = "wasm32")]
    {
        let user_connected_wallet = wasm::user_connected_wallet();
        game_state.wallet.address = user_connected_wallet;

        let game_mode = wasm::mode();
        let nft_link = wasm::nft_link();

        if game_mode == "player" {
            game_state.game_type = GameType::VisualNovelPlayer;
            game_state.player_nft_url = Some(nft_link);
        }
    }
}

fn load_cards(
    narrative_cards_handle: Res<NarrativeCardsHandle>,
    narrative_cards_assets: Res<Assets<NarrativeCards>>,
    character_cards_handle: Res<CharacterCardsHandle>,
    character_cards_assets: Res<Assets<CharacterCards>>,
    psychosis_cards_handle: Res<PsychosisCardsHandle>,
    psychosis_cards_assets: Res<Assets<PsychosisCards>>,
    mut game_state: ResMut<GameState>,
    mut app_state: ResMut<NextState<AppState>>,
    mut ew_load_nft: EventWriter<EventLoadNFTRequest>,
) {
    if let Some(narrative_cards) = narrative_cards_assets.get(narrative_cards_handle.id())
        && let Some(character_cards) = character_cards_assets.get(character_cards_handle.id())
        && let Some(psychosis_cards) = psychosis_cards_assets.get(psychosis_cards_handle.id())
    {
        let mut deck: Vec<VNCard> = vec![];
        for (i, narrative_card) in narrative_cards.iter().enumerate() {
            deck.push(VNCard {
                filename: format!("narrative-cards/card-{}.png", i + 1),
                metadata: VNCardMetadata::Narrative(
                    i + 1,
                    narrative_card.card_type.clone(),
                    narrative_card.genre.clone(),
                    narrative_card.name.clone(),
                    narrative_card.effect.clone(),
                    narrative_card.price,
                ),
            });
        }

        for (i, narrative_card) in character_cards.iter().enumerate() {
            deck.push(VNCard {
                filename: format!("character-cards/card-{}.png", i + 1),
                metadata: VNCardMetadata::Character(
                    i + 1,
                    narrative_card.name.clone(),
                    narrative_card.description.clone(),
                    narrative_card.price,
                ),
            });
        }

        for (i, psychosis_card) in psychosis_cards.iter().enumerate() {
            deck.push(VNCard {
                filename: format!("psychosis-cards/card-{}.png", i + 1),
                metadata: VNCardMetadata::Psychosis(
                    i + 1,
                    psychosis_card.name.clone(),
                    psychosis_card.description.clone(),
                ),
            });
        }

        game_state.game_deck = deck.clone();

        if game_state.game_type == GameType::VisualNovelPlayer {
            let nft_link = game_state.player_nft_url.clone().unwrap_or_default();
            ew_load_nft.send(EventLoadNFTRequest { url: nft_link });
            app_state.set(AppState::Loading3);
        } else {
            app_state.set(AppState::MainMenu);
        }
    }
}
