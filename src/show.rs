use crate::board::{Board, Piece};
use colored::Colorize;

pub fn print_pieces(board: Board, pieces: &Vec<Piece>, lines: usize) {
    let mut temp = vec!['-'; 50];
    let mut b = board.0 & crate::board::BOARD_MASK;

    let mut count = 0;
    while b != 0 {
        if b & 1 == 1 {
            temp[count] = '?';
        }
        count += 1;
        b >>= 1;
    }

    let mut map = vec![0, 1, 2, 3];
    let mut first = true;
    for piece in pieces {
        // for i in (0..4).rev() {
        //     for j in 0..10 {
        //         let color = match temp[i * 10 + j] {
        //             'I' => (31, 150, 221),
        //             'J' => (0, 71, 181),
        //             'T' => (148, 44, 154),
        //             'O' => (255, 202, 0),
        //             'S' => (82, 165, 41),
        //             'L' => (255, 96, 0),
        //             'Z' => (205, 28, 40),
        //             '?' => (196, 196, 196),
        //             _ => (30, 30, 30),
        //         };
        //         print!("{}", "  ".on_truecolor(color.0, color.1, color.2));
        //     }
        //     println!("");
        // }

        b = piece.as_bits();

        for i in 0..4 {
            let name = match first {
                true => 'x',
                false => piece.shape.name(),
            };
            for j in (0..10).filter(|j| (b >> (i * 10 + j)) & 1 != 0) {
                temp[map[i] * 10 + j] = name;
            }
            let mut full = true;
            for _ in (0..10).filter(|j| temp[map[i] * 10 + j] == '-') {
                full = false;
                break;
            }
            if full {
                let temp = map[i];
                map.remove(i);
                map.insert(0, temp);
            }
        }
        first = false;
    }
    for _ in 0..lines {
        println!("{}", "                    ".on_truecolor(0, 0, 0));
    }

    for i in (lines..4).rev() {
        for j in 0..10 {
            let color = match temp[i * 10 + j] {
                'I' => (31, 150, 221),
                'J' => (0, 71, 181),
                'T' => (148, 44, 154),
                'O' => (255, 202, 0),
                'S' => (82, 165, 41),
                'L' => (255, 96, 0),
                'Z' => (205, 28, 40),
                'x' => (255, 255, 255),
                '?' => (0, 0, 0),
                _ => (0, 0, 0),
            };
            print!("{}", "  ".on_truecolor(color.0, color.1, color.2));
        }
        println!();
    }
}
