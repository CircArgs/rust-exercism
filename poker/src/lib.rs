use counter::Counter;
use lazy_static::lazy_static;
use std::char;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
enum Suit {
    S,
    C,
    D,
    H,
}

impl PartialOrd for Suit {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        Some(Ordering::Equal)
    }
}
impl Ord for Suit {
    fn cmp(&self, _other: &Self) -> Ordering {
        Ordering::Equal
    }
}
#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Rank {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

lazy_static! {
    static ref SEQRANKMAP: HashMap<Rank, Rank> = {
        use Rank::*;
        let ranks = [
            Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace,
        ];
        let mut m = HashMap::new();
        for (r1, r2) in ranks[1..].iter().zip(ranks.iter()) {
            m.insert(*r1, *r2);
        }
        m
    };
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
struct Card(Suit, Rank);
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    Straight,
    Flush,
    FullHouse,
    FourKind,
    StraightFlush,
}

impl Card {
    fn from_str(s: &str) -> Self {
        if s.len() > 2 {
            Card(Card::suit_from_char(s.chars().nth(2).unwrap()), Rank::Ten)
        } else {
            Card(
                Card::suit_from_char(s.chars().nth(1).unwrap()),
                Card::rank_from_char(s.chars().nth(0).unwrap()),
            )
        }
    }
    fn rank_from_char(c: char) -> Rank {
        use Rank::*;
        match c {
            '2' => Two,
            '3' => Three,
            '4' => Four,
            '5' => Five,
            '6' => Six,
            '7' => Seven,
            '8' => Eight,
            '9' => Nine,
            'J' => Jack,
            'Q' => Queen,
            'K' => King,
            'A' => Ace,
            _ => unreachable!(),
        }
    }
    fn suit_from_char(c: char) -> Suit {
        use Suit::*;
        match c {
            'S' => S,
            'D' => D,
            'C' => C,
            'H' => H,
            _ => unreachable!(),
        }
    }
}
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    rank: HandRank,
    cards: Vec<Card>,
}

///make new hands by getting their rank and appropriately sorting their cards
impl Hand {
    fn new(cards: Vec<Card>) -> Self {
        use HandRank::*;
        use Rank::*;
        let mut cards = cards;

        let rank_counts = cards.iter().map(|c| c.1).collect::<Counter<_>>();
        cards.sort_unstable_by(|a, b| (rank_counts[&b.1], b).cmp(&(rank_counts[&a.1], a)));
        let hand_rank = match rank_counts.len() {
            2 => {
                let temp = rank_counts
                    .keys()
                    .filter(|k| rank_counts[k] == 4)
                    .collect::<Vec<_>>();
                if temp.len() > 0 {
                    FourKind
                } else {
                    FullHouse
                }
            }
            3 => {
                let temp = rank_counts
                    .keys()
                    .filter(|k| rank_counts[k] == 3)
                    .collect::<Vec<_>>();
                if temp.len() > 0 {
                    ThreeKind
                } else {
                    TwoPair
                }
            }
            4 => OnePair,
            _ => {
                let suits = cards.iter().map(|c| c.0).collect::<HashSet<_>>();
                let flush = suits.len() == 1;
                let straight = match cards.as_slice() {
                    [Card(_, Ace), Card(_, Five), Card(_, Four), Card(_, Three), Card(_, Two)] => {
                        let ace = cards.remove(0);
                        cards.push(ace);
                        true
                    }
                    _ => cards
                        .iter()
                        .zip(cards[1..].iter())
                        .all(|(i, j)| SEQRANKMAP[&i.1] == j.1),
                };

                if straight && flush {
                    StraightFlush
                } else if straight {
                    Straight
                } else if flush {
                    Flush
                } else {
                    HighCard
                }
            }
        };

        Hand {
            rank: hand_rank,
            cards,
        }
    }
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.

/// Strategy: convert each &str hand to a `Hand` which is sortable by its poker rank and its cards
/// sort the hands and then take all the highest hands which are equal
pub fn winning_hands<'a>(hands: &[&'a str]) -> Option<Vec<&'a str>> {
    let mut card_hands = hands
        .iter()
        .map(|s| {
            (
                s,
                Hand::new(
                    s.split_ascii_whitespace()
                        .map(Card::from_str)
                        .collect::<Vec<_>>(),
                ),
            )
        })
        .collect::<Vec<_>>();

    card_hands.sort_unstable_by(|a, b| b.1.cmp(&a.1));
    println!("{:?}", card_hands);
    let ret = card_hands
        .iter()
        .filter(|p| {
            let t = &p.1;
            let t2 = &card_hands[0].1;
            t.cmp(&t2) == Ordering::Equal
        })
        .map(|p| *p.0)
        .collect::<Vec<_>>();
    Some(ret)
}
