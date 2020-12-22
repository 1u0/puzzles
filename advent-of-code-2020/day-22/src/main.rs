use std::collections::HashSet;
use std::collections::VecDeque;
use std::io::{self, BufRead};

fn load_decks() -> (Vec<usize>, Vec<usize>) {
    let mut player = 0;
    let mut deck1 = Vec::new();
    let mut deck2 = Vec::new();
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        if line == "Player 1:" {
            player = 1;
        } else if line == "Player 2:" {
            player = 2;
        } else if !line.is_empty() {
            let card = line.parse().unwrap();
            match player {
                1 => deck1.push(card),
                2 => deck2.push(card),
                _ => panic!("invalid player number"),
            }
        }
    }
    (deck1, deck2)
}

fn play_game<Iter>(deck1: Iter, deck2: Iter, recursive: bool) -> (bool, VecDeque<usize>)
where
    Iter: std::iter::Iterator<Item = usize>,
{
    let mut game_history = HashSet::new();
    let mut deck1: VecDeque<usize> = deck1.collect();
    let mut deck2: VecDeque<usize> = deck2.collect();
    while !deck1.is_empty() && !deck2.is_empty() {
        if recursive && !game_history.insert((deck1.clone(), deck2.clone())) {
            return (true, deck1); // encountered a game situation seen before - the player 1 wins by the rule.
        }
        let card1 = deck1.pop_front().unwrap();
        let card2 = deck2.pop_front().unwrap();
        let player1_wins_round = if recursive && card1 <= deck1.len() && card2 <= deck2.len() {
            play_game(
                deck1.iter().take(card1).cloned(),
                deck2.iter().take(card2).cloned(),
                recursive,
            )
            .0
        } else {
            card1 > card2
        };
        if player1_wins_round {
            deck1.push_back(card1);
            deck1.push_back(card2);
        } else {
            deck2.push_back(card2);
            deck2.push_back(card1);
        }
    }
    if deck2.is_empty() {
        (true, deck1)
    } else {
        (false, deck2)
    }
}

fn calculate_score<Iter: std::iter::DoubleEndedIterator<Item = usize>>(deck: Iter) -> i64 {
    deck.rev()
        .enumerate()
        .map(|(i, card)| (i + 1) as i64 * card as i64)
        .sum()
}

fn solve1(deck1: &[usize], deck2: &[usize]) {
    let winning_deck = play_game(deck1.iter().cloned(), deck2.iter().cloned(), false).1;
    let score: i64 = calculate_score(winning_deck.iter().cloned());
    println!("Result {}", score);
}

fn solve2(deck1: &[usize], deck2: &[usize]) {
    let winning_deck = play_game(deck1.iter().cloned(), deck2.iter().cloned(), true).1;
    let score: i64 = calculate_score(winning_deck.iter().cloned());
    println!("Result {}", score);
}

fn main() {
    let decks = load_decks();
    solve1(&decks.0, &decks.1);
    solve2(&decks.0, &decks.1);
}
