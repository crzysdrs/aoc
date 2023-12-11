use crate::Day;
#[allow(unused_imports)]
use std::collections::*;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum Card {
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    J,
    Q,
    K,
    A,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub enum JokerCard {
    J,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    T,
    Q,
    K,
    A,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Ord, Eq)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hand(Vec<Card>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JokerHand(Vec<JokerCard>);

impl Hand {
    fn hand_type(&self) -> HandType {
        let m = HashMap::new();
        let m = self.0.iter().fold(m, |mut state, c| {
            *state.entry(c).or_insert(0) += 1;
            state
        });

        if m.len() == 1 {
            HandType::FiveOfKind
        } else if m.iter().any(|(_k, v)| *v == 4) {
            HandType::FourOfKind
        } else if m.len() == 2 {
            HandType::FullHouse
        } else if m.iter().any(|(_k, v)| *v == 3) {
            HandType::ThreeOfKind
        } else if m.iter().filter(|(_k, v)| **v == 2).count() == 2 {
            HandType::TwoPair
        } else if m.iter().filter(|(_k, v)| **v == 2).count() == 1 {
            HandType::OnePair
        } else {
            assert_eq!(m.len(), 5);
            HandType::HighCard
        }
    }
}
impl JokerHand {
    fn hand_type(&self) -> HandType {
        let m = HashMap::new();
        let m = self.0.iter().fold(m, |mut state, c| {
            *state.entry(c).or_insert(0) += 1;
            state
        });

        let joker_count = *m.get(&JokerCard::J).unwrap_or(&0);
        let most_non_joker = m
            .iter()
            .filter(|(k, _v)| ***k != JokerCard::J)
            .map(|(_k, v)| *v)
            .max()
            .unwrap_or(0);

        if most_non_joker + joker_count == 5 {
            HandType::FiveOfKind
        } else if most_non_joker + joker_count == 4 {
            HandType::FourOfKind
        } else if m.len() == 2 || (m.len() == 3 && joker_count > 0) {
            HandType::FullHouse
        } else if most_non_joker + joker_count == 3 {
            HandType::ThreeOfKind
        } else if m.iter().filter(|(_k, v)| **v == 2).count() == 2 {
            HandType::TwoPair
        } else if most_non_joker + joker_count == 2 {
            HandType::OnePair
        } else {
            assert_eq!(m.len(), 5);
            HandType::HighCard
        }
    }
}

use std::cmp::Ordering;
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            Ordering::Equal => self
                .0
                .iter()
                .zip(other.0.iter())
                .map(|(x, y)| x.cmp(y))
                .find(|c| *c != Ordering::Equal)
                .unwrap_or(Ordering::Equal),
            v => v,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for JokerHand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            Ordering::Equal => self
                .0
                .iter()
                .zip(other.0.iter())
                .map(|(x, y)| x.cmp(y))
                .find(|c| *c != Ordering::Equal)
                .unwrap_or(Ordering::Equal),
            v => v,
        }
    }
}

impl PartialOrd for JokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::str::FromStr for Card {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::A),
            "K" => Ok(Self::K),
            "Q" => Ok(Self::Q),
            "J" => Ok(Self::J),
            "T" => Ok(Self::T),
            "9" => Ok(Self::N9),
            "8" => Ok(Self::N8),
            "7" => Ok(Self::N7),
            "6" => Ok(Self::N6),
            "5" => Ok(Self::N5),
            "4" => Ok(Self::N4),
            "3" => Ok(Self::N3),
            "2" => Ok(Self::N2),
            _ => Err(()),
        }
    }
}

impl std::str::FromStr for JokerCard {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::A),
            "K" => Ok(Self::K),
            "Q" => Ok(Self::Q),
            "J" => Ok(Self::J),
            "T" => Ok(Self::T),
            "9" => Ok(Self::N9),
            "8" => Ok(Self::N8),
            "7" => Ok(Self::N7),
            "6" => Ok(Self::N6),
            "5" => Ok(Self::N5),
            "4" => Ok(Self::N4),
            "3" => Ok(Self::N3),
            "2" => Ok(Self::N2),
            _ => Err(()),
        }
    }
}

pub struct Solution {}
impl Day for Solution {
    const DAY: u32 = 7;
    type Input1 = Vec<(Hand, usize)>;
    type Input2 = Vec<(JokerHand, usize)>;
    type Sol1 = usize;
    type Sol2 = usize;

    fn process_input1(s: &str) -> Self::Input1 {
        s.lines()
            .map(|s| {
                let (cards, bid) = s.split_once(' ').unwrap();
                let cards = Hand(
                    cards
                        .chars()
                        .map(|c| c.to_string().parse().unwrap())
                        .collect(),
                );
                let bid = bid.parse().unwrap();
                (cards, bid)
            })
            .collect()
    }
    fn process_input2(s: &str) -> Self::Input2 {
        s.lines()
            .map(|s| {
                let (cards, bid) = s.split_once(' ').unwrap();
                let cards = JokerHand(
                    cards
                        .chars()
                        .map(|c| c.to_string().parse().unwrap())
                        .collect(),
                );
                let bid = bid.parse().unwrap();
                (cards, bid)
            })
            .collect()
    }
    fn p1(v: &Self::Input1) -> Self::Sol1 {
        let mut v = v.clone();
        v.sort_by_key(|(h, _b)| h.clone());

        v.iter().zip(1..).map(|((_h, b), i)| i * b).sum()
    }
    fn p2(v: &Self::Input2) -> Self::Sol2 {
        let mut v = v.clone();
        v.sort_by_key(|(h, _b)| h.clone());

        v.iter().zip(1..).map(|((_h, b), i)| i * b).sum()
    }
}

crate::default_tests!(253205868, 253907829);
crate::string_tests!(
    [(
        foo_sol1,
        "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        6440
    )],
    [(
        foo_sol2,
        "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483",
        5905
    )]
);
