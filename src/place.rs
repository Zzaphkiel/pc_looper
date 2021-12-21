// This mod is used to get all places a specific piece can be placed.

use crate::board::{Board, Piece, Shape};
use bitvec::prelude::{bitvec, BitVec};
use std::collections::BTreeSet;

pub struct PiecePlacer {
    // the board before the piece placed.
    board: Board,

    // the frontier queue for BFS.
    queue: Vec<Piece>,

    // the Vec records the piece stage vistied, to avoid visiting the same stage twice.
    visited: BitVec,

    // the placement already returend.
    returened: BTreeSet<u64>,
}

impl PiecePlacer {
    pub fn new(board: Board, shape: Shape) -> PiecePlacer {
        let piece = Piece::new(shape);
        let queue = vec![piece];
        let mut visited = bitvec![0; 0x4000];
        let returened = BTreeSet::new();

        visited.set(piece.pack() as usize, true);

        PiecePlacer {
            board,
            queue,
            visited,
            returened,
        }
    }
}

impl Iterator for PiecePlacer {
    // (piece placement data, board after piece placed)
    type Item = (Piece, Board);

    // use BFS to find all places it can be placed. In order to avoid returning the same placement of one piece,
    // I use a binary search tree 'returned' to record placements already returned.
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let piece = self.queue.pop()?;

            // try all movements of a piece stage and push new stage to the frontier queue.
            for &new_piece in &[
                piece.left(self.board),
                piece.right(self.board),
                piece.down(self.board),
                piece.cw(self.board),
                piece.ccw(self.board),
            ] {
                if !self.visited[new_piece.pack() as usize] {
                    self.visited.set(new_piece.pack() as usize, true);
                    self.queue.push(new_piece);
                }
            }

            let bits = piece.as_bits();
            if piece.can_place(self.board) && !self.returened.contains(&bits) {
                self.returened.insert(bits);
                return Some((piece, piece.place(self.board)));
            }
        }
    }
}
