mod board_state;
mod other_bits;

use crate::board_state::BoardState;
use crate::other_bits::Player;
use crate::other_bits::Player::{Dog, Hare};
use std::collections::HashMap;

fn main() {
    let board = BoardState::new();
    let mut play_count: usize = 0;
    let mut memo_pad: HashMap<BoardState, Player> = HashMap::new();
    let winner = play_game(&board, &mut play_count, &mut memo_pad);
    println!("the winner is {:?} after {} recursions", winner, play_count);
    println!(
        "the proportion of {:?} wins in memoization is {}",
        winner,
        memo_pad.values().filter(|&&p| p == winner).count() as f64 / memo_pad.len() as f64
    );
}

fn play_game(
    board: &BoardState,
    play_count: &mut usize,
    memo_pad: &mut HashMap<BoardState, Player>,
) -> Player {
    *play_count += 1;
    if board.turn_num() > 30 || board.hare_passed() {
        return Hare;
    }
    for m in board.get_moves() {
        let new_board = board.do_move(&m);
        match memo_pad.get(&new_board) {
            Some(&winner) => return winner,
            None => (),
        }
        let winner = play_game(&new_board, play_count, memo_pad);
        memo_pad.insert(new_board, winner);
        if winner == board.whose_turn() {
            return winner;
        }
    }
    // The player loses
    match board.whose_turn() {
        Dog => Hare,
        Hare => Dog,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board_state::BoardState;
    use crate::other_bits::Position::*;
    use std::collections::HashMap;

    #[test]
    fn test_play_game() {
        let b = BoardState {
            dogs: [Downdog, Upcenter, Downcenter],
            hare: Midcenter,
            turn: Hare,
            turn_num: 5,
        };
        let w = play_game(&b, &mut 0, &mut HashMap::new());
        assert_eq!(w, Hare);

        let mut b = BoardState::new();
        b.turn_num = 29;
        let w = play_game(&b, &mut 0, &mut HashMap::new());
        assert_eq!(w, Hare);

        b.dogs = [Uphare, Midcenter, Downhare];
        let w = play_game(&b, &mut 0, &mut HashMap::new());
        assert_eq!(w, Dog);
    }
}
