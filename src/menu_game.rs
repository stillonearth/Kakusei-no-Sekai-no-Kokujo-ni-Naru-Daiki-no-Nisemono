use bevy::prelude::*;

use bevy_hui::prelude::*;
use bevy_kira_audio::*;
use bevy_novel::events::EventSwitchNextNode;

use crate::{AppState, EventEndCardGame, EventPlayHand, GameState, GameType};

pub struct GameMenuPlugin;

impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), show_menu)
            .add_systems(OnExit(AppState::Game), despawn_menu)
            .add_systems(OnEnter(AppState::NovelPlayer), show_menu)
            .add_systems(OnExit(AppState::NovelPlayer), despawn_menu)
            .add_systems(
                Update,
                (render_ui, refresh_ui).run_if(in_state(AppState::Game)),
            )
            .add_systems(
                Update,
                (render_ui, refresh_ui).run_if(in_state(AppState::NovelPlayer)),
            )
            .add_event::<EventHideMainMenu>()
            .add_event::<EventShowMainMenu>()
            .add_event::<EventRefreshUI>()
            .add_event::<EventRenderUI>();
    }
}

#[derive(Component)]
pub struct GameMenu {}

#[derive(Event)]
pub struct EventHideMainMenu {}

#[derive(Event)]
pub struct EventShowMainMenu {}

/// Update Menu disaply variables
#[derive(Event, PartialEq, Eq)]
pub enum EventRefreshUI {
    PokerMenu(PokerMenuSettings),
    NovelMenu(String),
    ShopMenu,
    LoadingMenu,
    Narrative(NarrativeMenuSettings),
    GameOver(usize),
}

/// Despawn previous menu template and render a new one
#[derive(Event, PartialEq, Eq, Default, Debug)]
pub enum EventRenderUI {
    Poker(PokerMenuSettings),
    #[default]
    Novel,
    Shop,
    Loading,
    Narrative,
    GameOver,
}

#[derive(Event, PartialEq, Eq, Default, Debug)]
pub struct PokerMenuSettings {
    pub show_advance_button: bool,
    pub show_score: bool,
    pub score: usize,
}

#[derive(Event, PartialEq, Eq, Default, Debug)]
pub struct NarrativeMenuSettings {
    pub show_advance_button: bool,
}

pub fn show_menu(
    mut commands: Commands,
    mut html_funcs: HtmlFunctions,
    asset_server: Res<AssetServer>,
    game_state: Res<GameState>,
) {
    // menu
    commands.spawn((
        HtmlNode(asset_server.load("menu/novel_menu.html")),
        TemplateProperties::default()
            .with("advance_button_display", "flex")
            .with("score_display", "none")
            .with("score", &format!("{}", game_state.score))
            .with("title", ""),
        GameMenu {},
    ));

    // game menu button handlers
    html_funcs.register(
        "advance",
        |In(_),
         mut ew_switch_next_node: EventWriter<EventSwitchNextNode>,
         mut ew_play_hand: EventWriter<EventPlayHand>,
         mut ew_end_game: EventWriter<EventEndCardGame>,
         game_state: ResMut<GameState>| {
            if game_state.game_type == GameType::CardShop
                || game_state.game_type == GameType::Poker
                || game_state.game_type == GameType::Narrative
            {
                ew_play_hand.send(EventPlayHand {});
                ew_end_game.send(EventEndCardGame {});
            }
            ew_switch_next_node.send(EventSwitchNextNode {});
        },
    );
}

fn despawn_menu(
    mut commands: Commands,
    q_main_menu_entities: Query<(Entity, &GameMenu)>,
    audio: Res<Audio>,
) {
    for (entity, _) in q_main_menu_entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    audio.stop();
}

fn render_ui(
    mut commands: Commands,
    mut er_refresh_ui: EventReader<EventRenderUI>,
    q_game_menu: Query<(Entity, &GameMenu)>,
    asset_server: Res<AssetServer>,
    game_state: Res<GameState>,
) {
    for event in er_refresh_ui.read() {
        for (entity, _) in q_game_menu.iter() {
            commands.entity(entity).despawn_recursive();
        }

        match event {
            EventRenderUI::Poker(_) => {
                commands.spawn((
                    HtmlNode(asset_server.load("menu/poker_menu.html")),
                    TemplateProperties::default(),
                    GameMenu {},
                    Name::new("poker menu"),
                ));
            }
            EventRenderUI::Novel => {
                commands.spawn((
                    HtmlNode(asset_server.load("menu/novel_menu.html")),
                    TemplateProperties::default()
                        .with("advance_button_display", "flex")
                        .with("score_display", "none")
                        .with("score", &format!("{}", game_state.score))
                        .with("title", ""),
                    GameMenu {},
                    Name::new("novel menu"),
                ));
            }
            EventRenderUI::Shop => {
                commands.spawn((
                    HtmlNode(asset_server.load("menu/shop_menu.html")),
                    TemplateProperties::default().with("score", &format!("{}", game_state.score)),
                    GameMenu {},
                    Name::new("shop menu"),
                ));
            }
            EventRenderUI::Loading => {
                commands.spawn((
                    HtmlNode(asset_server.load("menu/loading_menu.html")),
                    TemplateProperties::default()
                        .with("advance_button_display", "none")
                        .with("score_display", "none")
                        .with("score", &format!("{}", game_state.score))
                        .with("title", "GENERATING CHAPTER"),
                    GameMenu {},
                    Name::new("loading menu"),
                ));
            }
            EventRenderUI::Narrative => {
                commands.spawn((
                    HtmlNode(asset_server.load("menu/narrative_menu.html")),
                    TemplateProperties::default(),
                    GameMenu {},
                    Name::new("narative menu"),
                ));
            }
            EventRenderUI::GameOver => {
                commands.spawn((
                    HtmlNode(asset_server.load("menu/game_over.html")),
                    TemplateProperties::default(),
                    GameMenu {},
                    Name::new("game over menu"),
                ));
            }
        }
    }
}

fn refresh_ui(
    mut er_refresh_ui: EventReader<EventRefreshUI>,
    mut q_text_labels: Query<(Entity, &mut Text, &Tags)>,
    mut q_nodes: Query<(Entity, &mut Node, &Tags)>,
    mut style: Query<&mut HtmlStyle>,
    game_state: Res<GameState>,
) {
    for event in er_refresh_ui.read() {
        match event {
            EventRefreshUI::PokerMenu(poker_menu_settings) => {
                for (entity, mut node, tags) in q_nodes.iter_mut() {
                    if let Some(marker) = tags.get("marker")
                        && marker == "button_advance"
                    {
                        node.display = match poker_menu_settings.show_advance_button {
                            true => Display::Flex,
                            false => Display::None,
                        };

                        if let Ok(mut style) = style.get_mut(entity) {
                            style.computed.node.display = node.display;
                        }
                    }
                }

                for (_, mut text, tags) in q_text_labels.iter_mut() {
                    if let Some(marker) = tags.get("marker")
                        && marker == "text_score"
                    {
                        *text = Text::new(format!("${}", poker_menu_settings.score));
                    }
                }
            }
            EventRefreshUI::ShopMenu => {
                for (_, mut text, tags) in q_text_labels.iter_mut() {
                    if let Some(marker) = tags.get("marker")
                        && marker == "text_score"
                    {
                        *text = Text::new(format!("${}", game_state.score));
                    }
                }
            }
            EventRefreshUI::NovelMenu(title) => {
                for (_, mut text, tags) in q_text_labels.iter_mut() {
                    if let Some(marker) = tags.get("marker")
                        && marker == "text_title"
                    {
                        *text = Text::new(title);
                    }
                }
            }
            EventRefreshUI::Narrative(narrative_menu_settings) => {
                for (entity, mut node, tags) in q_nodes.iter_mut() {
                    if let Some(marker) = tags.get("marker")
                        && marker == "button_advance"
                    {
                        node.display = match narrative_menu_settings.show_advance_button {
                            true => Display::Flex,
                            false => Display::None,
                        };

                        if let Ok(mut style) = style.get_mut(entity) {
                            style.computed.node.display = node.display;
                        }
                    }
                }
            }
            EventRefreshUI::LoadingMenu => {
                for (entity, mut node, tags) in q_nodes.iter_mut() {
                    if let Some(marker) = tags.get("marker")
                        && marker == "button_advance"
                    {
                        node.display = Display::Flex;

                        if let Ok(mut style) = style.get_mut(entity) {
                            style.computed.node.display = node.display;
                        }
                    }
                }
            }
            EventRefreshUI::GameOver(nft_id) => {
                for (_, mut text, tags) in q_text_labels.iter_mut() {
                    if let Some(marker) = tags.get("marker")
                        && marker == "text_minting_status"
                    {
                        *text = Text::new(format!("nft minted to your wallet. id: {}", *nft_id));
                    }
                }
            }
        }
    }
}
