use rand::prelude::*;
use std::collections::VecDeque;

#[derive(Debug, PartialEq, PartialOrd, Eq, Clone)]
struct Card(u8);

/// Simulates a game of Blackjack and determines the outcome.
pub fn play() -> &'static str {
    let mut deck = create_deck();
    let (player_1, player_2) = deal_cards(&mut deck);  // Removed 'mut' from player_1 and player_2

    let score_1 = player_1.iter().map(|card| card.0 as i64).sum::<i64>();
    let score_2 = player_2.iter().map(|card| card.0 as i64).sum::<i64>();

    // Determine the game result based on Blackjack rules
    if score_1 > 21 && score_2 > 21 || score_1 == score_2 {
        "Draw!"
    } else if score_1 > 21 || (score_2 <= 21 && score_2 > score_1) {
        "Player 2"
    } else {
        "Player 1"
    }
}

/// Creates a deck of 52 cards (13 ranks * 4 suits).
fn create_deck() -> Vec<Card> {
    let mut out = Vec::new();
    for idx in 1..14 {  // Cards 1-13 representing Ace to King
        for _ in 1..5 {  // Four suits for each rank
            out.push(Card(idx));
        }
    }
    out
}

/// Deals two cards to each player from a shuffled deck.
fn deal_cards(deck: &mut Vec<Card>) -> (VecDeque<Card>, VecDeque<Card>) {
    let mut player_1: VecDeque<Card> = VecDeque::new();
    let mut player_2: VecDeque<Card> = VecDeque::new();
    let mut rng = thread_rng();
    deck.shuffle(&mut rng);

    // Each player gets two cards initially
    for _ in 0..2 {
        player_1.push_back(deck.pop().expect("Deck is empty but dealing continues"));
        player_2.push_back(deck.pop().expect("Deck is empty but dealing continues"));
    }
    (player_1, player_2)
}
