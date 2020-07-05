use crate::other_bits::Player::{Dog, Hare};
use crate::other_bits::Position::*;
use crate::other_bits::*;
use std::hash::{Hash, Hasher};

#[derive(Debug, Eq, Clone)]
pub(crate) struct BoardState {
    pub(crate) dogs: [Position; 3],
    pub(crate) hare: Position,
    pub(crate) turn_num: usize,
    pub(crate) turn: Player,
}

impl BoardState {
    pub(crate) fn new() -> Self {
        BoardState {
            dogs: [Updog, Dogmost, Downdog],
            hare: Haremost,
            turn_num: 1,
            turn: Dog,
        }
    }

    pub(crate) fn whose_turn(&self) -> Player {
        self.turn
    }

    pub(crate) fn turn_num(&self) -> usize {
        self.turn_num
    }

    pub(crate) fn do_move(&self, Move(from, to): &Move) -> BoardState {
        match self.turn {
            Dog => {
                let mut dogs = self.dogs.clone();
                dogs[dogs.iter().position(|p| p == from).unwrap()] = *to;
                BoardState {
                    dogs,
                    turn: Hare,
                    turn_num: self.turn_num + 1,
                    ..*self
                }
            }
            Hare => BoardState {
                hare: *to,
                turn: Dog,
                turn_num: self.turn_num + 1,
                dogs: self.dogs.clone(),
            },
        }
    }

    pub(crate) fn get_moves(&self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        match self.turn {
            Dog => {
                for dog in self.dogs.iter() {
                    for mv in self.get_dog_moves(&dog) {
                        moves.push(mv);
                    }
                }
            }
            Hare => moves = self.get_hare_moves(),
        }
        moves
    }

    fn get_dog_moves(&self, pos: &Position) -> Vec<Move> {
        match pos {
            Dogmost => vec![Updog, Middog, Downdog],
            Updog => vec![Middog, Upcenter, Midcenter],
            Middog => vec![Updog, Downdog, Midcenter],
            Downdog => vec![Middog, Midcenter, Downcenter],
            Upcenter => vec![Midcenter, Uphare],
            Midcenter => vec![Upcenter, Downcenter, Uphare, Midhare, Downhare],
            Downcenter => vec![Midcenter, Downhare],
            Uphare => vec![Midhare, Haremost],
            Midhare => vec![Uphare, Downhare, Haremost],
            Downhare => vec![Midhare, Haremost],
            Haremost => vec![],
        }
        .iter()
        .filter(|p| self.is_empty(p))
        .map(|p| Move(*pos, *p))
        .collect()
    }

    fn get_hare_moves(&self) -> Vec<Move> {
        match self.hare {
            Dogmost => vec![Updog, Middog, Downdog],
            Updog => vec![Dogmost, Middog, Upcenter, Midcenter],
            Middog => vec![Dogmost, Updog, Downdog, Midcenter],
            Downdog => vec![Dogmost, Middog, Midcenter, Downcenter],
            Upcenter => vec![Updog, Midcenter, Uphare],
            Midcenter => vec![
                Updog, Middog, Downdog, Upcenter, Downcenter, Uphare, Midhare, Downhare,
            ],
            Downcenter => vec![Downdog, Midcenter, Downhare],
            Uphare => vec![Upcenter, Midcenter, Midhare, Haremost],
            Midhare => vec![Midcenter, Uphare, Downhare, Haremost],
            Downhare => vec![Downcenter, Midhare, Haremost],
            Haremost => vec![Uphare, Midhare, Downhare],
        }
        .iter()
        .filter(|p| self.is_empty(p))
        .map(|p| Move(self.hare, *p))
        .collect()
    }

    pub(crate) fn hare_passed(&self) -> bool {
        let ranks = |&pos| match pos {
            Dogmost => 0,
            Updog | Middog | Downdog => 1,
            Upcenter | Midcenter | Downcenter => 2,
            Uphare | Midhare | Downhare => 3,
            Haremost => 4,
        };
        self.dogs
            .iter()
            .map(ranks)
            .filter(|&r| r < ranks(&self.hare))
            .count()
            == 0
    }

    fn is_empty(&self, at: &Position) -> bool {
        self.hare != *at && !self.dogs.contains(at)
    }
}

impl PartialEq for BoardState {
    fn eq(&self, other: &Self) -> bool {
        self.dogs.iter().all(|p| other.dogs.contains(p))
            && self.hare == other.hare
            && self.turn == other.turn
            && self.turn_num == other.turn_num
    }
}

impl Hash for BoardState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut dogs = self.dogs.clone();
        dogs.sort_unstable();
        dogs.iter().for_each(|&p| p.hash(state));
        self.hare.hash(state);
        self.turn.hash(state);
        self.turn_num.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use crate::board_state::BoardState;
    use crate::other_bits::Move;
    use crate::other_bits::Player::{Dog, Hare};
    use crate::other_bits::Position::{
        Dogmost, Downcenter, Downdog, Downhare, Haremost, Midcenter, Middog, Midhare, Upcenter,
        Updog, Uphare,
    };
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    #[test]
    fn test_hare_passed() {
        let mut b = BoardState::new();
        assert!(!b.hare_passed());

        b.hare = Middog;
        assert!(!b.hare_passed());

        b.dogs[b.dogs.iter().position(|&p| p == Dogmost).unwrap()] = Upcenter;
        assert!(b.hare_passed());
    }

    #[test]
    fn test_is_empty() {
        let b = BoardState::new();
        assert!(b.is_empty(&Midcenter));
        assert!(!b.is_empty(&Haremost));
        assert!(!b.is_empty(&Dogmost));
    }

    #[test]
    fn test_equality() {
        let b1 = BoardState::new();
        let mut b2: BoardState = b1.clone();
        b2.dogs.rotate_left(1);
        assert_eq!(b1, b2);
    }

    #[test]
    fn test_hash() {
        let b1 = BoardState::new();
        let mut b2: BoardState = b1.clone();
        b2.dogs.rotate_left(1);
        let mut h1 = DefaultHasher::new();
        let mut h2 = h1.clone();
        b1.hash(&mut h1);
        b2.hash(&mut h2);
        assert_eq!(h1.finish(), h2.finish());
    }

    #[test]
    fn test_do_move() {
        let b0 = BoardState::new();
        let b1 = b0.do_move(&Move(Updog, Upcenter));
        assert_eq!(
            b1,
            BoardState {
                dogs: [Dogmost, Downdog, Upcenter],
                hare: Haremost,
                turn: Hare,
                turn_num: 2,
            }
        );
        assert_eq!(b1.turn_num, 2);

        let b2 = b1.do_move(&Move(Haremost, Midhare));
        assert_eq!(
            b2,
            BoardState {
                dogs: [Dogmost, Downdog, Upcenter],
                hare: Midhare,
                turn: Dog,
                turn_num: 3,
            }
        );
        assert_eq!(b2.turn_num, 3);
    }

    #[test]
    fn test_get_moves() {
        let b0 = BoardState::new();
        let moves = b0.get_moves();
        assert_eq!(moves.len(), 7);
        for mv in [
            Move(Dogmost, Middog),
            Move(Updog, Upcenter),
            Move(Updog, Midcenter),
            Move(Downdog, Downcenter),
            Move(Downdog, Midcenter),
            Move(Downdog, Middog),
            Move(Updog, Middog),
        ]
        .iter()
        {
            assert!(moves.contains(mv));
        }

        let b1 = b0.do_move(&Move(Dogmost, Middog));
        let moves = b1.get_moves();
        assert_eq!(moves.len(), 3);
        for mv in [
            Move(Haremost, Uphare),
            Move(Haremost, Midhare),
            Move(Haremost, Downhare),
        ]
        .iter()
        {
            assert!(moves.contains(mv));
        }
    }
}
