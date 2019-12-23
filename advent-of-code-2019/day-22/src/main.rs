use std::io;
use std::io::BufRead;

#[derive(Debug, Clone)]
enum Shuffle {
    Reverse,              // == deal into new stack
    Cut { n: i64 },       // == cut N
    DealBy { step: i64 }, // == deal with increment N
}

static REVERSE: &str = "deal into new stack";
static PREFIX_CUT_N: &str = "cut ";
static PREFIX_DEAL_WITH_INCREMENT_N: &str = "deal with increment ";

fn parse_shuffle(text: &str) -> Option<Shuffle> {
    if text == REVERSE {
        Some(Shuffle::Reverse)
    } else if text.starts_with(PREFIX_CUT_N) {
        text.trim_start_matches(PREFIX_CUT_N)
            .parse()
            .map(|n| Shuffle::Cut { n })
            .ok()
    } else if text.starts_with(PREFIX_DEAL_WITH_INCREMENT_N) {
        text.trim_start_matches(PREFIX_DEAL_WITH_INCREMENT_N)
            .parse()
            .map(|step| Shuffle::DealBy { step })
            .ok()
    } else {
        println!("Debug: {}", text);
        None
    }
}

fn load_shuffles() -> Vec<Shuffle> {
    io::stdin()
        .lock()
        .lines()
        .map(|line| parse_shuffle(&line.unwrap()).unwrap())
        .collect()
}

fn mul(a: i64, b: i64, modulo: i64) -> i64 {
    let res = ((a as i128) * (b as i128)) % (modulo as i128);
    res as i64
}

fn normalize(total_cards: i64, shuffles: &[Shuffle]) -> Vec<Shuffle> {
    let mut deal_by: i64 = 1;
    let mut cut = 0;
    let mut reverse = false;
    for shuffle in shuffles {
        match shuffle {
            Shuffle::Reverse => {
                reverse = !reverse;
            }
            Shuffle::Cut { n } => {
                let n = if reverse { -n } else { *n };
                cut = (cut + n) % total_cards;
            }
            Shuffle::DealBy { step } => {
                deal_by = mul(deal_by, *step, total_cards);
                cut = mul(cut, *step, total_cards);
                if reverse {
                    cut = (cut - step + 1 + total_cards) % total_cards;
                }
            }
        }
    }
    let mut normalized_shuffles = Vec::new();
    if deal_by != 1 {
        normalized_shuffles.push(Shuffle::DealBy { step: deal_by });
    }
    if cut != 0 {
        normalized_shuffles.push(Shuffle::Cut { n: cut });
    }
    if reverse {
        normalized_shuffles.push(Shuffle::Reverse);
    }
    normalized_shuffles
}

fn repeat(total_cards: i64, shuffles: &[Shuffle], count: i64) -> Vec<Shuffle> {
    let mut shuffles = normalize(total_cards, shuffles);
    let mut result = Vec::new();
    let mut count = count;
    while count > 0 {
        if count % 2 == 1 {
            result.extend(shuffles.iter().cloned());
        }
        let mut shuffles_2 = shuffles.to_vec();
        shuffles_2.extend(shuffles.iter().cloned());
        shuffles = normalize(total_cards, &shuffles_2);
        count /= 2;
    }
    normalize(total_cards, &result)
}

fn multiplicative_inverse(num: i64, modulo: i64) -> i64 {
    let mut r0 = num;
    let mut r1 = modulo;
    let mut s0 = 1;
    let mut s1 = 0;
    while r1 != 0 {
        let q = r0 / r1;
        let r = r0 % r1;
        let s = s0 - q * s1;
        s0 = s1;
        s1 = s;
        r0 = r1;
        r1 = r;
    }
    s0
}

fn invert(total_cards: i64, shuffles: &[Shuffle]) -> Vec<Shuffle> {
    shuffles
        .iter()
        .rev()
        .map(|shuffle| match shuffle {
            Shuffle::Reverse => Shuffle::Reverse,
            Shuffle::Cut { n } => Shuffle::Cut { n: -n },
            Shuffle::DealBy { step } => Shuffle::DealBy {
                step: multiplicative_inverse(*step, total_cards),
            },
        })
        .collect()
}

fn track_card(total_cards: i64, card: i64, shuffles: &[Shuffle]) -> i64 {
    let mut position = card;
    for shuffle in shuffles {
        position = match shuffle {
            Shuffle::Reverse => total_cards - position - 1,
            Shuffle::Cut { n } => (position - n + total_cards) % total_cards,
            Shuffle::DealBy { step } => mul(position, *step, total_cards),
        }
    }
    position
}

fn solve1() {
    let total_cards = 10007;
    let shuffles = load_shuffles();
    let shuffles = normalize(total_cards, &shuffles);
    let result = track_card(total_cards, 2019, &shuffles);
    println!("Result for task 1: {:?}", result);
}

fn solve2() {
    let total_cards = 119_315_717_514_047;
    let shuffles = load_shuffles();
    let shuffles = repeat(total_cards, &shuffles, 101741582076661);
    let shuffles = invert(total_cards, &shuffles);
    let result = track_card(total_cards, 2020, &shuffles);
    println!("Result for task 2: {:?}", result);
}

fn main() {
    solve2();
}
