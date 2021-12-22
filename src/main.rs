mod board;
mod place;
mod ppt;
mod search;
mod show;

extern crate process_memory;
#[cfg(windows)]
extern crate winapi;

use board::{Board, Piece, Shape};
use ppt::{get_pid, Ppt};
use process_memory::*;
use search::search_pc;
use show::print_pieces;
use std::{
    process::Command,
    sync::mpsc::{channel, Sender},
    thread, // time,
};

enum BoardEvent {
    // board, current_piece, next_pieces, hold
    Continue(Board, Shape, Vec<Shape>, Option<Shape>),
    Exit,
}

fn read_from_ppt(send: Sender<BoardEvent>, ppt_pid: process_memory::Pid) {
    let process_handler: ProcessHandle = ppt_pid.try_into_process_handle().unwrap();
    let ppt = Ppt {
        process_handle: process_handler,
    };
    let mut prev_queue: Vec<Shape> = vec![];
    let mut prev_piece: Option<Shape> = None;
    let mut prev_hold: Option<Shape> = None;
    let mut prev_board: Option<u64> = None;

    while ppt.still_active().unwrap() {
        let mut bit_board: u64 = 0;

        let board = match ppt.get_columns() {
            Ok(b) => b,
            Err(_) => continue,
        };

        let next_pieces = match ppt.get_next_pieces() {
            Ok(next) => next,
            Err(_) => continue,
        };

        let mut queue: Vec<Shape> = next_pieces
            .iter()
            .map(|val| Shape::from_ppt(*val))
            .collect();

        let bag_pieces = match ppt.get_pieces_from_bags() {
            Ok(bag) => bag,
            Err(_) => continue,
        };

        let piece_count = match ppt.get_piece_count() {
            Ok(count) => count,
            Err(_) => continue,
        };

        let bag: Vec<Shape> = bag_pieces.iter().map(|val| Shape::from_ppt(*val)).collect();
        for i in piece_count..piece_count + 7 {
            queue.push(bag[i as usize]);
        }

        let hold = match ppt.get_hold() {
            Ok(val) => Some(Shape::from_ppt(val)),
            Err(_) => None,
        };

        for i in (0..4).rev() {
            for j in (0..10).rev() {
                bit_board <<= 1;
                bit_board |= match board[j][i] {
                    -1 => 0,
                    _ => 1,
                };
            }
        }

        let current_piece = match ppt.get_current_piece() {
            Some(piece) => Shape::from_ppt(piece),
            None => continue,
        };

        if prev_board == Some(bit_board) && prev_hold == hold {
            continue;
        }

        if prev_queue == queue && prev_hold == hold {
            continue;
        }

        if prev_piece == Some(current_piece) {
            continue;
        }

        prev_board = Some(bit_board);
        prev_hold = hold;
        prev_piece = Some(current_piece);
        prev_queue = queue.clone();

        send.send(BoardEvent::Continue(
            Board(bit_board),
            current_piece,
            queue,
            hold,
        ))
        .ok();
    }

    send.send(BoardEvent::Exit).ok();
}

fn main() {
    let ppt_pid = get_pid("puyopuyotetris.exe");

    let (board_send, board_recv) = channel();
    thread::spawn(move || read_from_ppt(board_send, ppt_pid));

    let mut prev_solution: Vec<Piece> = vec![];
    loop {
        match board_recv.recv().unwrap() {
            BoardEvent::Continue(bit_board, current_piece, queue, hold) => {
                if !bit_board.is_empty() && !prev_solution.is_empty() {
                    prev_solution.remove(0);
                    if let Some(i) = bit_board.soln_can_pc(&prev_solution) {
                        let _ = Command::new("cmd.exe").arg("/c").arg("cls").status();
                        print_pieces(bit_board.add_lines(i), &prev_solution, i);
                        continue;
                    }
                }

                let res = search_pc(bit_board, current_piece, queue, hold);
                let _ = Command::new("cmd.exe").arg("/c").arg("cls").status();
                print_pieces(bit_board.add_lines(res.1), &res.0, res.1);
                prev_solution = res.0;
            }

            BoardEvent::Exit => break,
        }
    }

    // let board = Board(0b1111101111_1111000111_1111111111_1111111111);
    // for (new_piece, _) in place::PiecePlacer::new(board, Shape::T) {
    //     println!("{:?}", new_piece);

    //     print_pieces(board, &vec![new_piece], 0);
    // }
}
