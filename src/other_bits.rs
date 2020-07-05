use std::cmp::Ordering;

#[derive(Eq, PartialEq, PartialOrd, Ord, Hash, Copy, Clone, Debug)]
pub(crate) enum Position {
    Dogmost,
    Updog,
    Middog,
    Downdog,
    Upcenter,
    Midcenter,
    Downcenter,
    Uphare,
    Midhare,
    Downhare,
    Haremost,
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub(crate) enum Player {
    Dog,
    Hare,
}

#[derive(PartialEq)]
pub(crate) struct Move(pub Position, pub Position); // from, to

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.0.partial_cmp(&other.0) {
            Some(Ordering::Equal) => self.1.partial_cmp(&other.1),
            otherwise => otherwise,
        }
    }
}
