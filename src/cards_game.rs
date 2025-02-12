use std::cmp::Ordering;
use std::fmt;

use anyhow::Result;
use bevy::asset::Asset;
use bevy::prelude::{Deref, DerefMut};
use bevy::reflect::TypePath;
use bevy::render::render_resource::encase::rts_array::Length;
use bevy::utils::HashMap;
use bevy_la_mesa::CardMetadata;
use serde::Deserialize;

#[derive(Clone, Debug, Default)]
pub(crate) struct VNCard {
    pub(crate) filename: String,
    pub(crate) metadata: VNCardMetadata,
}

#[derive(Deserialize, Clone)]
pub struct NarrativeCard {
    pub name: String,
    pub card_type: String,
    pub genre: String,
    pub effect: String,
    pub flavor_text: String,
    pub price: u16,
}

#[derive(Deserialize, Clone)]
pub struct CharacterCard {
    pub name: String,
    pub description: String,
    pub flavor_text: String,
    pub price: u16,
    pub filename: String,
}

#[derive(Deserialize, Asset, TypePath, Deref, DerefMut)]
pub struct CharacterCards(pub Vec<CharacterCard>);

#[derive(Deserialize, Asset, TypePath, Deref, DerefMut)]
pub struct NarrativeCards(pub Vec<NarrativeCard>);

#[derive(Clone, Debug)]
pub(crate) enum VNCardMetadata {
    // value, suit
    Poker(u8, String),
    // index, card_type, genre, name, effect, price
    Narrative(usize, String, String, String, String, u16),
    // index, name, description, price
    Character(usize, String, String, u16),
}

impl Default for VNCardMetadata {
    fn default() -> Self {
        VNCardMetadata::Poker(0, "".to_string())
    }
}

#[allow(dead_code)]
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
        if let VNCardMetadata::Narrative(_index, card_type, _genre, _name, _effect, _price) = self {
            return Some(card_type.clone());
        }
        if let VNCardMetadata::Character(_, _, _, _) = self {
            return Some("character".to_string());
        }
        None
    }

    pub(crate) fn genre(&self) -> Option<String> {
        if let VNCardMetadata::Narrative(_index, _card_type, genre, _name, _effect, _price) = self {
            return Some(genre.clone());
        }
        None
    }

    pub(crate) fn effect(&self) -> Option<String> {
        if let VNCardMetadata::Narrative(_index, _card_type, _genre, _name, effect, _price) = self {
            return Some(effect.clone());
        }
        None
    }

    pub(crate) fn name(&self) -> Option<String> {
        if let VNCardMetadata::Narrative(_index, _card_type, _genre, name, _effect, _price) = self {
            return Some(name.clone());
        }
        if let VNCardMetadata::Character(_, name, _description, _price) = self {
            return Some(name.clone());
        }
        None
    }

    pub(crate) fn description(&self) -> Option<String> {
        if let VNCardMetadata::Character(_, _name, description, _price) = self {
            return Some(description.clone());
        }
        None
    }

    pub(crate) fn price(&self) -> Option<u16> {
        if let VNCardMetadata::Narrative(_index, _card_type, _genre, _name, _effect, price) = self {
            return Some(*price);
        }
        None
    }

    pub(crate) fn is_narrative(&self) -> bool {
        if let VNCardMetadata::Narrative(_index, _card_type, _genre, _name, _effect, _price) = self
        {
            return true;
        }
        false
    }

    pub(crate) fn is_character(&self) -> bool {
        if let VNCardMetadata::Character(_index, _name, _description, _price) = self {
            return true;
        }
        false
    }
}

#[allow(clippy::vec_init_then_push)]
pub(crate) fn load_poker_deck() -> Vec<VNCard> {
    let mut deck: Vec<VNCard> = vec![];

    // Hearts
    for value in 2..11 {
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

pub(crate) fn filter_initial_narrative_cards(deck: Vec<VNCard>) -> Vec<VNCard> {
    deck.iter()
        .filter(|card| {
            card.metadata.is_narrative() && card.metadata.price().unwrap_or_default() <= 30
        })
        .cloned()
        .collect()
}

pub(crate) fn filter_initial_character_cards(deck: Vec<VNCard>) -> Vec<VNCard> {
    deck.iter()
        .filter(|card| {
            card.metadata.is_character() && card.metadata.price().unwrap_or_default() <= 30
        })
        .cloned()
        .collect()
}

fn filter_narrative_cards_by_type(deck: Vec<VNCard>, tp: String) -> Result<Vec<VNCard>> {
    let cards: Vec<VNCard> = deck
        .iter()
        .filter(|card| {
            card.metadata.is_narrative() && card.metadata.card_type().unwrap_or_default() == tp
        })
        .cloned()
        .collect();
    Ok(cards)
}

pub fn filer_narrative_setting_deck(deck: Vec<VNCard>) -> Result<Vec<VNCard>> {
    filter_narrative_cards_by_type(deck, "setting".to_string())
}

pub fn filter_narrative_plot_twist_deck(deck: Vec<VNCard>) -> Result<Vec<VNCard>> {
    filter_narrative_cards_by_type(deck, "plot twist".to_string())
}

pub fn filter_narrative_conflict_deck(deck: Vec<VNCard>) -> Result<Vec<VNCard>> {
    filter_narrative_cards_by_type(deck, "conflict".to_string())
}

pub fn filter_character_deck(deck: Vec<VNCard>) -> Result<Vec<VNCard>> {
    Ok(deck
        .iter()
        .filter(|card| card.metadata.is_character())
        .cloned()
        .collect())
}

impl CardMetadata for VNCard {
    type Output = VNCard;

    fn front_image_filename(&self) -> String {
        self.filename.clone()
    }

    fn back_image_filename(&self) -> String {
        match self.metadata {
            VNCardMetadata::Poker(_, _) => "poker-cards/Back_1.png".into(),
            VNCardMetadata::Narrative(_, _, _, _, _, _) => "poker-cards/Back_2.png".into(),
            VNCardMetadata::Character(_, _, _, _) => "poker-cards/Back_3.png".into(),
        }
    }
}

// -----------------------
// Poker Combination Tests
// -----------------------

fn is_flush(cards: &[VNCard]) -> bool {
    let suit = &cards[0].metadata.suit();
    if suit.is_none() {
        return false;
    }

    let is_sorted = cards.is_sorted_by_key(|card| card.metadata.value().unwrap_or_default());

    is_sorted && cards.iter().all(|card| card.metadata.suit() == *suit) && cards.length() == 5
}

fn is_straight(values: &[u8]) -> bool {
    (values.windows(2).all(|w| w[1] == w[0] + 1) || (values.contains(&2) && values.contains(&14)))
        && values.length() == 5
}

fn is_royal_flush(cards: &[VNCard]) -> bool {
    if !is_flush(cards) || cards.len() != 5 {
        return false;
    }
    let values: Vec<u8> = cards
        .iter()
        .map(|card| card.metadata.value().unwrap_or_default())
        .collect();

    values[0] == 10 && values[1] == 11 && values[2] == 12 && values[3] == 13 && values[4] == 14
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

    for (key, value) in counts.iter() {
        if *value == 4 {
            return (true, *key as u8);
        }
    }

    return (false, 0);
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
        let keys: Vec<u8> = counts.keys().map(|c| c.clone()).collect();
        let score: u8 = keys.iter().sum();

        (true, score)
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

    for (key, value) in counts.iter() {
        if *value == 3 {
            return (true, *key as u8);
        }
    }

    return (false, 0);
}

fn two_pair(cards: &[VNCard]) -> (bool, u8) {
    let mut counts = HashMap::new();

    // Count occurrences of each card value
    for card in cards {
        *counts
            .entry(card.metadata.value().unwrap_or_default())
            .or_insert(0) += 1;
    }

    // Filter values that occur exactly twice and sort them
    let two_pairs: Vec<_> = counts
        .iter()
        .filter(|&(_, &v)| v == 2)
        .map(|(&k, _)| k)
        .collect();

    // Check if there are exactly two pairs
    if two_pairs.len() == 2 {
        // Return true and the sum of the two pair values
        (true, two_pairs.iter().sum())
    } else {
        (false, 0)
    }
}

fn one_pair(cards: &[VNCard]) -> (bool, u8) {
    let mut counts = HashMap::new();

    // Count occurrences of each card value
    for card in cards {
        *counts
            .entry(card.metadata.value().unwrap_or_default())
            .or_insert(0) += 1;
    }

    // Find the value that occurs exactly twice
    let mut score = 0;
    let has_one_pair = counts.values().any(|&v| v == 2);

    if has_one_pair {
        // Find the first key with a value of 2 and set it as the score
        for (&key, &val) in &counts {
            if val == 2 {
                score = key;
                break;
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::SliceRandom;
    use rand::thread_rng;

    #[test]
    fn test_royal_flush() {
        let deck: Vec<VNCard> = load_poker_deck();

        let hearts: Vec<_> = deck
            .iter()
            .filter(|card| card.metadata.suit() == Some("Hearts".to_string()))
            .map(|card| card.clone())
            .collect();

        assert_eq!(hearts.len(), 13);

        let mut royal_flush_set: Vec<_> = hearts
            .iter()
            .filter(|card| card.metadata.value().unwrap() >= 10)
            .map(|card| card.clone())
            .collect();

        assert_eq!(royal_flush_set.len(), 5);

        assert!(is_royal_flush(&royal_flush_set));

        let mut rng = thread_rng();
        royal_flush_set.shuffle(&mut rng);

        assert!(!is_royal_flush(&royal_flush_set));
    }

    #[test]
    fn test_straight_flush() {
        let deck: Vec<VNCard> = load_poker_deck();

        let hearts: Vec<_> = deck
            .iter()
            .filter(|card| card.metadata.suit() == Some("Hearts".to_string()))
            .map(|card| card.clone())
            .collect();

        assert_eq!(hearts.len(), 13);

        let mut straight_flush_set: Vec<_> = hearts
            .iter()
            .filter(|card| card.metadata.value().unwrap() < 10)
            .map(|card| card.clone())
            .take(5)
            .collect();

        assert!(is_straight_flush(&straight_flush_set));

        let mut rng = thread_rng();
        straight_flush_set.shuffle(&mut rng);

        assert!(!is_straight_flush(&straight_flush_set));
    }

    #[test]
    fn test_four_of_kind() {
        let deck: Vec<VNCard> = load_poker_deck();

        let mut cards: Vec<_> = deck
            .iter()
            .filter(|card| card.metadata.value() == Some(5))
            .map(|card| card.clone())
            .collect();

        assert_eq!(cards.len(), 4);
        let extra_card = deck[0].clone();

        cards.push(extra_card);
        assert_eq!(cards.len(), 5);

        let (is_four_of_a_kind, score) = four_of_a_kind(&cards);

        assert!(is_four_of_a_kind);
        assert_eq!(score, 5);

        let mut rng = thread_rng();
        cards.shuffle(&mut rng);

        let (is_four_of_a_kind, score) = four_of_a_kind(&cards);

        assert!(is_four_of_a_kind);
        assert_eq!(score, 5);
    }

    #[test]
    fn test_full_house() {
        let deck: Vec<VNCard> = load_poker_deck();

        let fours: Vec<_> = deck
            .iter()
            .filter(|card| card.metadata.value() == Some(4))
            .map(|card| card.clone())
            .take(3)
            .collect();

        assert_eq!(fours.len(), 3);
        let twos: Vec<_> = deck
            .iter()
            .filter(|card| card.metadata.value() == Some(2))
            .map(|card| card.clone())
            .take(2)
            .collect();

        assert_eq!(twos.len(), 2);

        let mut cards = [fours, twos].concat();
        assert_eq!(cards.len(), 5);

        let (is_full_house, score) = full_house(&cards);
        assert!(is_full_house);
        assert_eq!(score, 4 + 2);

        let mut rng = thread_rng();
        cards.shuffle(&mut rng);

        let (is_full_house, score) = full_house(&cards);
        assert!(is_full_house);
        assert_eq!(score, 4 + 2);
    }

    #[test]
    fn test_flush() {
        let deck: Vec<VNCard> = load_poker_deck();

        let hearts: Vec<_> = deck
            .iter()
            .filter(|card| card.metadata.suit() == Some("Hearts".to_string()))
            .map(|card| card.clone())
            .collect();

        assert_eq!(hearts.len(), 13);

        let mut flush_set: Vec<_> = hearts
            .iter()
            .filter(|card| card.metadata.value().unwrap() > 2)
            .map(|card| card.clone())
            .take(5)
            .collect();

        assert_eq!(flush_set.len(), 5);

        assert!(is_flush(&flush_set));

        let mut rng = thread_rng();
        flush_set.shuffle(&mut rng);

        assert!(!is_flush(&flush_set));
    }

    #[test]
    fn test_straight() {
        // i don't select different suits in this test

        let deck: Vec<VNCard> = load_poker_deck();

        let hearts: Vec<_> = deck
            .iter()
            .filter(|card| card.metadata.suit() == Some("Hearts".to_string()))
            .map(|card| card.clone())
            .collect();

        assert_eq!(hearts.len(), 13);

        let mut straight_flush_set: Vec<_> = hearts
            .iter()
            .filter(|card| card.metadata.value().unwrap() < 10)
            .map(|card| card.clone())
            .take(5)
            .collect();

        assert!(straight(&straight_flush_set));

        let mut rng = thread_rng();
        straight_flush_set.shuffle(&mut rng);

        assert!(!straight(&straight_flush_set));
    }

    #[test]
    fn test_three_of_kind() {
        let deck: Vec<VNCard> = load_poker_deck();

        let mut cards: Vec<_> = deck
            .iter()
            .filter(|card| card.metadata.value() == Some(4))
            .map(|card| card.clone())
            .take(3)
            .collect();

        assert_eq!(cards.len(), 3);
        let extra_card = deck[0].clone();

        cards.push(extra_card);

        let extra_card = deck[1].clone();

        cards.push(extra_card);

        assert_eq!(cards.len(), 5);

        let (is_three_of_a_kind, score) = three_of_a_kind(&cards);

        assert!(is_three_of_a_kind);
        assert_eq!(score, 4);

        let mut rng = thread_rng();
        cards.shuffle(&mut rng);

        let (is_three_of_a_kind, score) = three_of_a_kind(&cards);

        assert!(is_three_of_a_kind);
        assert_eq!(score, 4);
    }

    #[test]
    fn test_two_pair() {
        let deck: Vec<VNCard> = load_poker_deck();

        let pair_1: Vec<_> = deck
            .iter()
            .filter(|card| card.metadata.value() == Some(6))
            .map(|card| card.clone())
            .take(2)
            .collect();
        assert_eq!(pair_1.len(), 2);

        let pair_2: Vec<_> = deck
            .iter()
            .filter(|card| card.metadata.value() == Some(7))
            .map(|card| card.clone())
            .take(2)
            .collect();
        assert_eq!(pair_2.len(), 2);

        let mut cards = [pair_1, pair_2].concat();
        assert_eq!(cards.len(), 4);

        let extra_card = deck[0].clone();
        cards.push(extra_card);
        assert_eq!(cards.len(), 5);

        let (is_two_pair, score) = two_pair(&cards);
        assert!(is_two_pair);
        assert_eq!(score, 6 + 7);

        let mut rng = thread_rng();
        cards.shuffle(&mut rng);

        let (is_two_pair, score) = two_pair(&cards);
        assert!(is_two_pair);
        assert_eq!(score, 6 + 7);
    }

    #[test]
    fn test_one_pair() {
        let deck: Vec<VNCard> = load_poker_deck();

        let pair_1: Vec<_> = deck
            .iter()
            .filter(|card| card.metadata.value() == Some(6))
            .map(|card| card.clone())
            .take(2)
            .collect();
        assert_eq!(pair_1.len(), 2);

        let pair_2: Vec<_> = deck
            .iter()
            .filter(|card| card.metadata.value() == Some(8))
            .map(|card| card.clone())
            .take(1)
            .collect();
        assert_eq!(pair_2.len(), 1);

        let pair_3: Vec<_> = deck
            .iter()
            .filter(|card| card.metadata.value() == Some(9))
            .map(|card| card.clone())
            .take(1)
            .collect();
        assert_eq!(pair_3.len(), 1);

        let mut cards = [pair_1, pair_2, pair_3].concat();
        assert_eq!(cards.len(), 4);

        let extra_card = deck[0].clone();
        cards.push(extra_card);
        assert_eq!(cards.len(), 5);

        let (is_one_pair, score) = one_pair(&cards);
        assert!(is_one_pair);
        assert_eq!(score, 6);

        let mut rng = thread_rng();
        cards.shuffle(&mut rng);

        let (is_one_pair, score) = one_pair(&cards);
        assert!(is_one_pair);
        assert_eq!(score, 6);
    }
}
