use crate::board::{Board, Piece, PieceQueue, Shape};
use crate::place::PiecePlacer;
use std::collections::{BTreeSet, BinaryHeap};

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Stage {
    evaluation: i16,
    bit_board: Board,
    piece: Shape,
    placed: Vec<Piece>,
    left: PieceQueue,
    hold: Option<Shape>,
}

pub fn search_pc(
    board: Board,
    piece: Shape,
    sequence: Vec<Shape>,
    hold: Option<Shape>,
) -> (Vec<Piece>, usize) {
    match board.0.count_ones() % 4 {
        0 => {
            for i in [2, 0] {
                let res = search_pc_impl(Stage {
                    evaluation: board.add_lines(i).evaluate(),
                    bit_board: board.add_lines(i),
                    piece: piece,
                    placed: vec![],
                    left: PieceQueue::new(sequence.clone()),
                    hold: hold,
                });
                if !res.is_empty() {
                    return (res, i);
                }
            }
        }
        2 => {
            for i in [3, 1] {
                let res = search_pc_impl(Stage {
                    evaluation: board.add_lines(i).evaluate(),
                    bit_board: board.add_lines(i),
                    piece: piece,
                    placed: vec![],
                    left: PieceQueue::new(sequence.clone()),
                    hold: hold,
                });
                if !res.is_empty() {
                    return (res, i);
                }
            }
        }
        _ => return (vec![], 0),
    }

    (vec![], 0)
}

fn search_pc_impl(stage: Stage) -> Vec<Piece> {
    if !stage.bit_board.is_pcable() {
        return vec![];
    }

    let mut visited: BTreeSet<Stage> = BTreeSet::new();
    let mut fronrier: BinaryHeap<Stage> = BinaryHeap::new();
    // let mut fronrier: Vec<Stage> = Vec::new();
    // let mut count = 0;
    fronrier.push(stage);
    while !fronrier.is_empty() {
        // count += 1;
        // println!("{}", count);

        let t = fronrier.pop().unwrap();

        for (new_piece, new_board) in PiecePlacer::new(t.bit_board, t.piece) {
            if !new_board.is_pcable() {
                continue;
            }
            let mut new_placed = t.placed.clone();
            new_placed.push(new_piece);

            if new_board.is_pc() {
                return new_placed;
            }

            if !t.left.is_empty() {
                let new_stage = Stage {
                    evaluation: new_board.evaluate(),
                    bit_board: new_board,
                    piece: t.left.now(),
                    placed: new_placed,
                    left: t.left.go(),
                    hold: t.hold,
                };

                if !visited.contains(&new_stage) {
                    visited.insert(new_stage.clone());
                    fronrier.push(new_stage);
                }
            } else if t.left.is_empty() && !t.hold.is_none() {
                let new_stage = Stage {
                    evaluation: new_board.evaluate(),
                    bit_board: new_board,
                    piece: t.hold.unwrap(),
                    placed: new_placed,
                    left: PieceQueue(0),
                    hold: None,
                };

                if !visited.contains(&new_stage) {
                    visited.insert(new_stage.clone());
                    fronrier.push(new_stage);
                }
            } else {
                return vec![];
            }
        }

        if t.left.is_empty() && t.hold.is_none() {
            continue;
        } else if (!t.left.is_empty()) && (t.hold.is_none()) {
            for (new_piece, new_board) in PiecePlacer::new(t.bit_board, t.left.now()) {
                if !new_board.is_pcable() {
                    continue;
                }

                let mut new_placed = t.placed.clone();
                new_placed.push(new_piece);

                if new_board.is_pc() {
                    return new_placed;
                }

                if t.left.go().is_empty() {
                    let new_stage = Stage {
                        evaluation: new_board.evaluate(),
                        bit_board: new_board,
                        piece: t.piece,
                        placed: new_placed,
                        left: PieceQueue(0),
                        hold: None,
                    };

                    if !visited.contains(&new_stage) {
                        visited.insert(new_stage.clone());
                        fronrier.push(new_stage);
                    }
                } else {
                    let new_stage = Stage {
                        evaluation: new_board.evaluate(),
                        bit_board: new_board,
                        piece: t.left.go().now(),
                        placed: new_placed,
                        left: t.left.go().go(),
                        hold: Some(t.piece),
                    };

                    if !visited.contains(&new_stage) {
                        visited.insert(new_stage.clone());
                        fronrier.push(new_stage);
                    }
                }
            }
        } else if (!t.left.is_empty()) && !t.hold.is_none() {
            for (new_piece, new_board) in PiecePlacer::new(t.bit_board, t.hold.unwrap()) {
                if !new_board.is_pcable() {
                    continue;
                }

                let mut new_placed = t.placed.clone();
                new_placed.push(new_piece);

                if new_board.is_pc() {
                    return new_placed;
                }

                let new_stage = Stage {
                    evaluation: new_board.evaluate(),
                    bit_board: new_board,
                    piece: t.left.now(),
                    placed: new_placed,
                    left: t.left.go(),
                    hold: Some(t.piece),
                };

                if !visited.contains(&new_stage) {
                    visited.insert(new_stage.clone());
                    fronrier.push(new_stage);
                }
            }
        } else {
            for (new_piece, new_board) in PiecePlacer::new(t.bit_board, t.hold.unwrap()) {
                if !new_board.is_pcable() {
                    continue;
                }

                let mut new_placed = t.placed.clone();
                new_placed.push(new_piece);

                if new_board.is_pc() {
                    return new_placed;
                }

                let new_stage = Stage {
                    evaluation: new_board.evaluate(),
                    bit_board: new_board,
                    piece: t.piece,
                    placed: new_placed,
                    left: PieceQueue(0),
                    hold: None,
                };

                if !visited.contains(&new_stage) {
                    visited.insert(new_stage.clone());
                    fronrier.push(new_stage);
                }
            }
        }
    }

    vec![]
}
