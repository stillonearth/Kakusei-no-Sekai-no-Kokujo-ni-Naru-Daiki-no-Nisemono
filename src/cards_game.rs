use std::cmp::Ordering;
use std::fmt;
use std::fs::File;
use std::io::Read;

use anyhow::Result;
use bevy::render::render_resource::encase::rts_array::Length;
use bevy::utils::HashMap;
use bevy_la_mesa::CardMetadata;
use serde::Deserialize;

#[derive(Clone, Debug, Default)]
pub(crate) struct VNCard {
    pub(crate) filename: String,
    pub(crate) metadata: VNCardMetadata,
}

#[derive(Deserialize)]
struct NarrativeCard {
    name: String,
    card_type: String,
    genre: String,
    effect: String,
    flavor_text: String,
}

#[derive(Clone, Debug)]
pub(crate) enum VNCardMetadata {
    // value, suit
    Poker(u8, String),
    // index, card_type, genre, name, effect
    Narrative(usize, String, String, String, String),
}

impl Default for VNCardMetadata {
    fn default() -> Self {
        VNCardMetadata::Poker(0, "".to_string())
    }
}

impl VNCardMetadata {
    pub(crate) fn suit(&self) -> Option<String> {
        if let VNCardMetadata::Poker(_, suit) = self {
            return Some(suit.clone());
        }
        None
    }

    pub(crate) fn value(&self) -> Option<u8> {
        if let VNCardMetadata::Poker(value, _) = self {
            return Some(*value);
        }
        None
    }

    pub(crate) fn card_type(&self) -> Option<String> {
        if let VNCardMetadata::Narrative(_index, card_type, _genre, _name, _effect) = self {
            return Some(card_type.clone());
        }
        None
    }

    pub(crate) fn genre(&self) -> Option<String> {
        if let VNCardMetadata::Narrative(_index, _card_type, genre, _name, _effect) = self {
            return Some(genre.clone());
        }
        None
    }

    pub(crate) fn effect(&self) -> Option<String> {
        if let VNCardMetadata::Narrative(_index, _card_type, _genre, _name, effect) = self {
            return Some(effect.clone());
        }
        None
    }

    pub(crate) fn name(&self) -> Option<String> {
        if let VNCardMetadata::Narrative(_index, _card_type, _genre, name, _effect) = self {
            return Some(name.clone());
        }
        None
    }
}

#[allow(clippy::vec_init_then_push)]
pub(crate) fn load_poker_deck() -> Vec<VNCard> {
    let mut deck: Vec<VNCard> = vec![];

    // Hearts
    for value in 2..10 {
        let filename = format!("poker-cards/Hearts_{}.png", value);
        let metadata = VNCardMetadata::Poker(value, "Hearts".to_string());
        deck.push(VNCard { filename, metadata });
    }

    deck.push(VNCard {
        filename: "poker-cards/Hearts_J.png".to_string(),
        metadata: VNCardMetadata::Poker(11, "Hearts".to_string()),
    });
    deck.push(VNCard {
        filename: "poker-cards/Hearts_Q.png".to_string(),
        metadata: VNCardMetadata::Poker(12, "Hearts".to_string()),
    });
    deck.push(VNCard {
        filename: "poker-cards/Hearts_K.png".to_string(),
        metadata: VNCardMetadata::Poker(13, "Hearts".to_string()),
    });
    deck.push(VNCard {
        filename: "poker-cards/Hearts_ACE.png".to_string(),
        metadata: VNCardMetadata::Poker(14, "Hearts".to_string()),
    });

    // Spades
    for value in 2..10 {
        let filename = format!("poker-cards/Spades_{}.png", value);
        let metadata = VNCardMetadata::Poker(value, "Spades".to_string());
        deck.push(VNCard { filename, metadata });
    }

    deck.push(VNCard {
        filename: "poker-cards/Spades_J.png".to_string(),
        metadata: VNCardMetadata::Poker(11, "Spades".to_string()),
    });
    deck.push(VNCard {
        filename: "poker-cards/Spades_Q.png".to_string(),
        metadata: VNCardMetadata::Poker(12, "Spades".to_string()),
    });
    deck.push(VNCard {
        filename: "poker-cards/Spades_K.png".to_string(),
        metadata: VNCardMetadata::Poker(13, "Spades".to_string()),
    });
    deck.push(VNCard {
        filename: "poker-cards/Spades_ACE.png".to_string(),
        metadata: VNCardMetadata::Poker(14, "Spades".to_string()),
    });

    // Clubs
    for value in 2..10 {
        let filename = format!("poker-cards/Clubs_{}.png", value);
        let metadata = VNCardMetadata::Poker(value, "Clubs".to_string());
        deck.push(VNCard { filename, metadata });
    }

    deck.push(VNCard {
        filename: "poker-cards/Clubs_J.png".to_string(),
        metadata: VNCardMetadata::Poker(11, "Clubs".to_string()),
    });
    deck.push(VNCard {
        filename: "poker-cards/Clubs_Q.png".to_string(),
        metadata: VNCardMetadata::Poker(12, "Clubs".to_string()),
    });
    deck.push(VNCard {
        filename: "poker-cards/Clubs_K.png".to_string(),
        metadata: VNCardMetadata::Poker(13, "Clubs".to_string()),
    });
    deck.push(VNCard {
        filename: "poker-cards/Clubs_ACE.png".to_string(),
        metadata: VNCardMetadata::Poker(14, "Clubs".to_string()),
    });

    // Clubs
    for value in 2..10 {
        let filename = format!("poker-cards/Diamonds_{}.png", value);
        let metadata = VNCardMetadata::Poker(value, "Diamonds".to_string());
        deck.push(VNCard { filename, metadata });
    }

    deck.push(VNCard {
        filename: "poker-cards/Diamonds_J.png".to_string(),
        metadata: VNCardMetadata::Poker(11, "Diamonds".to_string()),
    });
    deck.push(VNCard {
        filename: "poker-cards/Diamonds_Q.png".to_string(),
        metadata: VNCardMetadata::Poker(12, "Diamonds".to_string()),
    });
    deck.push(VNCard {
        filename: "poker-cards/Diamonds_K.png".to_string(),
        metadata: VNCardMetadata::Poker(13, "Diamonds".to_string()),
    });
    deck.push(VNCard {
        filename: "poker-cards/Diamonds_ACE.png".to_string(),
        metadata: VNCardMetadata::Poker(14, "Diamonds".to_string()),
    });

    deck
}

#[allow(clippy::vec_init_then_push)]
pub(crate) fn load_narrative_deck() -> Result<Vec<VNCard>> {
    let mut deck: Vec<VNCard> = vec![];

    let mut file = File::open("assets/narrative-cards/cards.json")?;
    let mut data = String::new();
    file.read_to_string(&mut data)?;

    let narrative_cards: Vec<NarrativeCard> = serde_json::from_str(&data)?;

    for (i, narrative_card) in narrative_cards.iter().enumerate() {
        deck.push(VNCard {
            filename: format!("narrative-cards/card-{}.png", i + 1),
            metadata: VNCardMetadata::Narrative(
                i + 1,
                narrative_card.card_type.clone(),
                narrative_card.genre.clone(),
                narrative_card.name.clone(),
                narrative_card.effect.clone(),
            ),
        });
    }

    Ok(deck)
}

fn load_narrative_cards_by_type(tp: String) -> Result<Vec<VNCard>> {
    let narrative_deck = load_narrative_deck()?;
    let cards: Vec<VNCard> = narrative_deck
        .iter()
        .filter(|card| card.metadata.card_type().unwrap_or_default() == tp).cloned()
        .collect();
    Ok(cards)
}

pub fn load_narrative_setting_deck() -> Result<Vec<VNCard>> {
    load_narrative_cards_by_type("Setting".to_string())
}

pub fn load_narrative_plot_twist_deck() -> Result<Vec<VNCard>> {
    load_narrative_cards_by_type("Plot Twist".to_string())
}

pub fn load_narrative_conflict_deck() -> Result<Vec<VNCard>> {
    load_narrative_cards_by_type("Conflict".to_string())
}

impl CardMetadata for VNCard {
    type Output = VNCard;

    fn front_image_filename(&self) -> String {
        self.filename.clone()
    }

    fn back_image_filename(&self) -> String {
        match self.metadata {
            VNCardMetadata::Poker(_, _) => "poker-cards/Back_1.png".into(),
            VNCardMetadata::Narrative(_, _, _, _, _) => "poker-cards/Back_2.png".into(),
        }
    }
}

fn is_flush(cards: &[VNCard]) -> bool {
    let suit = &cards[0].metadata.suit();
    if suit.is_none() {
        return false;
    }
    cards.iter().all(|card| card.metadata.suit() == *suit) && cards.length() == 5
}

fn is_straight(values: &[u8]) -> bool {
    (values.windows(2).all(|w| w[1] == w[0] + 1) || (values.contains(&2) && values.contains(&14)))
        && values.length() == 5
}

fn is_royal_flush(cards: &[VNCard]) -> bool {
    if !is_flush(cards) {
        return false;
    }
    let values: Vec<u8> = cards
        .iter()
        .map(|card| card.metadata.value().unwrap_or_default())
        .collect();
    values.contains(&10)
        && values.contains(&11)
        && values.contains(&12)
        && values.contains(&13)
        && values.contains(&14)
}

fn is_straight_flush(cards: &[VNCard]) -> bool {
    is_flush(cards)
        && is_straight(
            &cards
                .iter()
                .map(|card| card.metadata.value().unwrap_or_default())
                .collect::<Vec<u8>>(),
        )
        && cards.len() == 5
}

fn four_of_a_kind(cards: &[VNCard]) -> (bool, u8) {
    let mut counts = HashMap::new();
    for card in cards {
        *counts
            .entry(card.metadata.value().unwrap_or_default())
            .or_insert(0) += 1;
    }
    let mut score = 0;
    if let Some(&value) = counts.values().find(|&&v| v == 4) {
        score = value;
        (true, score)
    } else {
        (false, score)
    }
}

fn full_house(cards: &[VNCard]) -> (bool, u8) {
    let mut counts = HashMap::new();
    for card in cards {
        *counts
            .entry(card.metadata.value().unwrap_or_default())
            .or_insert(0) += 1;
    }
    let values: Vec<_> = counts.values().collect();
    if values.contains(&&3) && values.contains(&&2) {
        (true, *counts.keys().max().unwrap())
    } else {
        (false, 0)
    }
}

fn straight(cards: &[VNCard]) -> bool {
    is_straight(
        &cards
            .iter()
            .map(|card| card.metadata.value().unwrap_or_default())
            .collect::<Vec<u8>>(),
    )
}

fn three_of_a_kind(cards: &[VNCard]) -> (bool, u8) {
    let mut counts = HashMap::new();
    for card in cards {
        *counts
            .entry(card.metadata.value().unwrap_or_default())
            .or_insert(0) += 1;
    }
    let mut score = 0;
    if let Some(&value) = counts.values().find(|&&v| v == 3) {
        score = value;
        (true, score)
    } else {
        (false, score)
    }
}

fn two_pair(cards: &[VNCard]) -> (bool, u8) {
    let mut counts = HashMap::new();
    for card in cards {
        *counts
            .entry(card.metadata.value().unwrap_or_default())
            .or_insert(0) += 1;
    }
    let values: Vec<_> = counts.values().collect();
    if values.iter().filter(|&&v| *v == 2).count() == 2 {
        (true, *counts.keys().max().unwrap())
    } else {
        (false, 0)
    }
}

fn one_pair(cards: &[VNCard]) -> (bool, u8) {
    let mut counts = HashMap::new();
    for card in cards {
        *counts
            .entry(card.metadata.value().unwrap_or_default())
            .or_insert(0) += 1;
    }
    let mut score = 0;
    if let Some(&value) = counts.values().find(|&&v| v == 2) {
        score = value;
        (true, score)
    } else {
        (false, score)
    }
}

fn high_card(cards: &[VNCard]) -> u8 {
    cards
        .iter()
        .map(|card| card.metadata.value().unwrap_or_default())
        .max()
        .unwrap()
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum PokerCombination {
    RoyalFlush,
    StraightFlush,
    FourOfAKind,
    FullHouse,
    Flush,
    Straight,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl PartialOrd for PokerCombination {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for PokerCombination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PokerCombination::RoyalFlush => write!(f, "Royal Flush"),
            PokerCombination::StraightFlush => write!(f, "Straight Flush"),
            PokerCombination::FourOfAKind => write!(f, "Four of a Kind"),
            PokerCombination::FullHouse => write!(f, "Full House"),
            PokerCombination::Flush => write!(f, "Flush"),
            PokerCombination::Straight => write!(f, "Straight"),
            PokerCombination::ThreeOfAKind => write!(f, "Three of a Kind"),
            PokerCombination::TwoPair => write!(f, "Two Pair"),
            PokerCombination::OnePair => write!(f, "One Pair"),
            PokerCombination::HighCard => write!(f, "High Card"),
        }
    }
}

impl Ord for PokerCombination {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (PokerCombination::RoyalFlush, PokerCombination::RoyalFlush)
            | (PokerCombination::StraightFlush, PokerCombination::StraightFlush)
            | (PokerCombination::FourOfAKind, PokerCombination::FourOfAKind)
            | (PokerCombination::FullHouse, PokerCombination::FullHouse)
            | (PokerCombination::Flush, PokerCombination::Flush)
            | (PokerCombination::Straight, PokerCombination::Straight)
            | (PokerCombination::ThreeOfAKind, PokerCombination::ThreeOfAKind)
            | (PokerCombination::TwoPair, PokerCombination::TwoPair)
            | (PokerCombination::OnePair, PokerCombination::OnePair)
            | (PokerCombination::HighCard, PokerCombination::HighCard) => Ordering::Equal,

            (PokerCombination::RoyalFlush, _) => Ordering::Greater,
            (_, PokerCombination::RoyalFlush) => Ordering::Less,

            (PokerCombination::StraightFlush, _) => Ordering::Greater,
            (_, PokerCombination::StraightFlush) => Ordering::Less,

            (PokerCombination::FourOfAKind, _) => Ordering::Greater,
            (_, PokerCombination::FourOfAKind) => Ordering::Less,

            (PokerCombination::FullHouse, _) => Ordering::Greater,
            (_, PokerCombination::FullHouse) => Ordering::Less,

            (PokerCombination::Flush, _) => Ordering::Greater,
            (_, PokerCombination::Flush) => Ordering::Less,

            (PokerCombination::Straight, _) => Ordering::Greater,
            (_, PokerCombination::Straight) => Ordering::Less,

            (PokerCombination::ThreeOfAKind, _) => Ordering::Greater,
            (_, PokerCombination::ThreeOfAKind) => Ordering::Less,

            (PokerCombination::TwoPair, _) => Ordering::Greater,
            (_, PokerCombination::TwoPair) => Ordering::Less,

            (PokerCombination::OnePair, _) => Ordering::Greater,
            (_, PokerCombination::OnePair) => Ordering::Less,
        }
    }
}

pub(crate) fn check_poker_hand(cards: Vec<VNCard>) -> (PokerCombination, u8) {
    let sorted_cards = {
        let mut cards = cards;
        cards.sort_by_key(|k| k.metadata.value().unwrap_or_default());
        cards
    };

    let sum_value = sorted_cards
        .iter()
        .map(|k| k.metadata.value().unwrap_or_default())
        .collect::<Vec<u8>>()
        .iter()
        .sum();

    if is_royal_flush(&sorted_cards) {
        return (PokerCombination::RoyalFlush, sum_value);
    } else if is_straight_flush(&sorted_cards) {
        return (PokerCombination::StraightFlush, sum_value);
    }

    let (four_of_a_kind, score) = four_of_a_kind(&sorted_cards);
    if four_of_a_kind {
        return (PokerCombination::FourOfAKind, score);
    }

    let (full_house, score) = full_house(&sorted_cards);
    if full_house {
        return (PokerCombination::FullHouse, score);
    }

    if is_flush(&sorted_cards) {
        return (PokerCombination::Flush, high_card(&sorted_cards));
    }

    if straight(&sorted_cards) {
        return (
            PokerCombination::Straight,
            sorted_cards
                .last()
                .unwrap()
                .metadata
                .value()
                .unwrap_or_default(),
        );
    }

    let (three_of_a_kind, score) = three_of_a_kind(&sorted_cards);
    if three_of_a_kind {
        return (PokerCombination::ThreeOfAKind, score);
    }

    let (two_pair, score) = two_pair(&sorted_cards);
    if two_pair {
        return (PokerCombination::TwoPair, score);
    }

    let (one_pair, score) = one_pair(&sorted_cards);
    if one_pair {
        return (PokerCombination::OnePair, score);
    }

    (PokerCombination::HighCard, high_card(&sorted_cards))
}
