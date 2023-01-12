use std::collections::{HashMap, BinaryHeap};
use std::collections::HashSet;
use std::cmp::Reverse;


//copy from https://exercism.org/tracks/rust/exercises/poker/solutions/Hnasar
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct PokerHand{
	counts : Vec<usize>,
    cards : Vec<u8>,
}

impl From<&str> for PokerHand{
    fn from(s : &str) -> Self {
        let (cards, suits) : (Vec<u8>, Vec<u8>) = s.split_whitespace().map(parse_poker).unzip();
        assert!(cards.len() == 5);

        let mut count_by_card = HashMap::new();
        for card in cards{
            *count_by_card.entry(card).or_default() += 1;
        }

        let mut card_counts : Vec<_> = count_by_card.iter().collect();
        card_counts.sort_unstable_by_key(|(&c, &num)| Reverse((num, c)));

        let (mut cards, mut counts) : (Vec<_>, Vec<_>) = card_counts.into_iter().unzip();

        if counts.len() == 5 {
            if cards == [14, 5, 4, 3, 2] {
                cards = vec![5,4,3,2,1];
            }

            let is_flush = suits.iter().all(|&s| s == suits[0]);
            let is_straight = cards[0] - cards[cards.len() -1] == 4;

            counts = match (is_flush, is_straight) {
                (true, true) => vec![4, 2],  // straight flush is best, 4 of a kind[4, 1], full house is [3, 2]
                (true, false) => vec![3, 1, 3], // flush is after [3, 2], before straight kind[3,1,2]
                (false, true) => vec![3, 1, 2], //straight better than [2,2,1], [2,1,1,1]
                _ => counts,  //other like [2,2,1], [2,1,1,1], [1,1,1,1,1]
            };
        }

        Self { counts, cards  }
    }
}

fn parse_poker(poker : &str) -> (u8, u8) {
    let (card_str, suit_str) = poker.split_at(poker.len() -1);
    let card : u8 = match card_str.parse() {
        Ok(card) => card,
        Err(_) => "JQKA".find(card_str).unwrap() as u8 + 11,
    };
    (card, suit_str.as_bytes()[0])
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut h : BinaryHeap<_> = hands.iter().map(|&s| (PokerHand::from(s), s)).collect();
    let mut ret = Vec::new();

    if let Some((best_hand, s)) = h.pop(){
        ret.push(s);

        while let Some((hand, s)) = h.pop() {
            if hand < best_hand {
                break;
            }
            ret.push(s);
        }
    }
    ret
}
