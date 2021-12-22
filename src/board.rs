/// Sorry for my confusing English.

/// This mod implements the rules of Tetris game to move, place, or rotate a piece.

/// In this part, the most of code references Wirelyre's code:
/// https://github.com/wirelyre/tetra-tools/blob/main/basic/src/gameplay.rs
/// A thousand thanks to Wirelyre.

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Board(pub u64);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Piece {
    pub shape: Shape,
    pub col: i8,
    pub row: i8,
    pub rotation: Rotation,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Shape {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum Rotation {
    None,
    Clockwise,
    Half,
    ConterClockwise,
}

impl Board {
    pub fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub fn has_isolated_cell(self) -> bool {
        let full = (self.0 >> 30) & (self.0 >> 20) & (self.0 >> 10) & (self.0 >> 0);
        let not_empty = (self.0 >> 30) | (self.0 >> 20) | (self.0 >> 10) | (self.0 >> 0);

        let bounded = {
            let left_bounded = (self.0 << 1) | 0b0000000001_0000000001_0000000001_0000000001;
            let right_bounded = (self.0 >> 1) | 0b1000000000_1000000000_1000000000_1000000000;
            let bounded_cells = (left_bounded & right_bounded) | self.0;

            (bounded_cells >> 30)
                & (bounded_cells >> 20)
                & (bounded_cells >> 10)
                & (bounded_cells >> 0)
        };

        (not_empty & !full & bounded) != 0
    }

    pub fn has_imbalanced_split(self) -> bool {
        let col_0: u64 = 0b0000000001_0000000001_0000000001_0000000001;
        let col_1: u64 = col_0 << 1;
        let col_2: u64 = col_0 << 2;
        let col_3: u64 = col_0 << 3;
        let col_4: u64 = col_0 << 4;
        let col_5: u64 = col_0 << 5;
        let col_6: u64 = col_0 << 6;
        let col_7: u64 = col_0 << 7;

        let left_0: u64 = col_0;
        let left_1: u64 = left_0 | col_1;
        let left_2: u64 = left_1 | col_2;
        let left_3: u64 = left_2 | col_3;
        let left_4: u64 = left_3 | col_4;
        let left_5: u64 = left_4 | col_5;
        let left_6: u64 = left_5 | col_6;
        let left_7: u64 = left_6 | col_7;

        fn check_col(board: Board, col_mask: u64, left_mask: u64) -> bool {
            if (board.0 | (board.0 >> 1)) & col_mask == col_mask {
                let left = board.0 & left_mask;

                if left.count_ones() % 4 != 0 {
                    return true;
                }
            }
            false
        }

        false
            || check_col(self, col_1, left_1)
            || check_col(self, col_2, left_2)
            || check_col(self, col_3, left_3)
            || check_col(self, col_4, left_4)
            || check_col(self, col_5, left_5)
            || check_col(self, col_6, left_6)
            || check_col(self, col_7, left_7)
    }

    pub fn is_pcable(self) -> bool {
        !self.has_imbalanced_split() && !self.has_isolated_cell() && (self.0 >> 40 == 0)
    }

    pub fn lines_cleared(self) -> i16 {
        let lines = [
            0b1111111111,
            0b1111111111_1111111111,
            0b1111111111_1111111111_1111111111,
            0b1111111111_1111111111_1111111111_1111111111,
        ];

        for i in (0..3).rev() {
            if (self.0 & lines[i]) == lines[i] {
                return (i + 1) as i16;
            }
        }

        0
    }

    pub fn piece_placed(self) -> i16 {
        ((self.0 & BOARD_MASK).count_ones() / 4) as i16
    }

    pub fn row_transition(self) -> i16 {
        let mut res = 0;
        let mut row_mask = 0b1111111111;
        let mut prev = 0b1111111111;

        for _ in 0..4 {
            res += ((self.0 & row_mask) ^ prev).count_ones();
            prev = (self.0 & row_mask) << 10;
            row_mask <<= 10;
        }

        res as i16
    }

    pub fn column_transition(self) -> i16 {
        let mut res = 0;
        let mut col_mask = 0b0000000001_0000000001_0000000001_0000000001;
        let mut prev = 0b0000000001_0000000001_0000000001_0000000001;

        for _ in 0..10 {
            res += ((self.0 & col_mask) ^ prev).count_ones();
            prev = (self.0 & col_mask) << 1;
            col_mask <<= 1;
        }

        res += (prev ^ col_mask).count_ones();
        res as i16
    }

    pub fn evaluate(self) -> i16 {
        let depth = self.piece_placed();
        let lines_cleared = self.lines_cleared();
        let row_transition = self.row_transition();
        let col_transition = self.column_transition();

        (depth << 1) + lines_cleared - row_transition * 30 - col_transition * 15
    }

    pub fn is_pc(self) -> bool {
        self.0 == BOARD_MASK
    }

    // To solve 3-line, 2-line and 1-line PC. I add some full lines to board to 4 lines.
    pub fn add_lines(self, n: usize) -> Board {
        let lines = [
            0,
            0b_1111111111,
            0b_1111111111_1111111111,
            0b_1111111111_1111111111_1111111111,
        ];

        if n == 0 {
            self
        } else {
            Board((self.0 << (n * 10)) | lines[n])
        }
    }

    // This function is used to check whether a PC solution is valid
    // The value returned represents the number of cleared lines.
    pub fn soln_can_pc(self, solution: &Vec<Piece>) -> Option<usize> {
        for i in 0..4 {
            let mut board = self.add_lines(i);
            if !board.is_pcable() {
                continue;
            }
            let mut flag = true;
            for piece in solution {
                if piece.can_place(board) {
                    board = piece.place(board);
                    if !board.is_pcable() {
                        flag = false;
                        continue;
                    }
                } else {
                    flag = false;
                    break;
                }
            }

            if flag && board.is_pc() {
                return Some(i);
            }
        }

        None
    }
}

impl Piece {
    pub fn new(shape: Shape) -> Piece {
        Piece {
            shape,
            col: 0,
            row: 4,
            rotation: Rotation::None,
        }
    }

    pub fn pack(self) -> u16 {
        ((self.rotation as u16) << 12)
            | ((self.shape as u16) << 8)
            | ((self.col as u16) << 4)
            | ((self.row as u16) << 0)
    }

    pub fn as_bits(self) -> u64 {
        let shift = self.row * 10 + self.col;
        PIECE_SHAPES[self.shape as usize][self.rotation as usize] << shift
    }

    fn collides_in(self, board: Board) -> bool {
        (self.as_bits() & board.0) != 0
    }

    pub fn can_place(self, board: Board) -> bool {
        let bits = self.as_bits();
        ((bits & BOARD_MASK) != 0) && ((bits & !BOARD_MASK) == 0) && self.down(board) == self
    }

    pub fn place(self, board: Board) -> Board {
        debug_assert!(self.can_place(board));
        debug_assert!((board.0 & self.as_bits()) == 0);

        let mut unordered_board = board.0 | self.as_bits();

        let mut ordered_board = 0;
        let mut complete_lines = 0;
        let mut complete_lines_shift = 0;

        for _ in 0..4 {
            let this_line = (unordered_board >> 30) & 0b1111111111;
            unordered_board <<= 10;

            if this_line == 0b1111111111 {
                complete_lines <<= 10;
                complete_lines |= this_line;
                complete_lines_shift += 10;
            } else {
                ordered_board <<= 10;
                ordered_board |= this_line;
            }
        }

        ordered_board <<= complete_lines_shift;
        ordered_board |= complete_lines;

        Board(ordered_board)
    }

    pub fn left(self, board: Board) -> Piece {
        let mut new = self;
        new.col -= 1;

        if (new.col < 0) || new.collides_in(board) {
            self
        } else {
            new
        }
    }

    pub fn right(self, board: Board) -> Piece {
        let mut new = self;
        new.col += 1;
        let max_col = PIECE_MAX_COLS[self.shape as usize][self.rotation as usize];

        if (new.col > max_col) || new.collides_in(board) {
            self
        } else {
            new
        }
    }

    pub fn down(self, board: Board) -> Piece {
        let mut new = self;
        new.row -= 1;

        if (new.row < 0) || new.collides_in(board) {
            self
        } else {
            new
        }
    }

    fn in_bounds(self) -> bool {
        let max_col = PIECE_MAX_COLS[self.shape as usize][self.rotation as usize];

        (self.col >= 0) && (self.col <= max_col) && (self.row >= 0) && (self.row <= 5)
    }

    pub fn cw(self, board: Board) -> Piece {
        let rotation = self.rotation.cw();

        let kicks = &KICKS[self.shape as usize][self.rotation as usize];
        for (kick_col, kick_row) in kicks {
            let new = Piece {
                shape: self.shape,
                col: self.col + kick_col,
                row: self.row + kick_row,
                rotation: rotation,
            };

            if new.in_bounds() && !new.collides_in(board) {
                return new;
            }
        }

        self
    }

    pub fn ccw(self, board: Board) -> Piece {
        let rotation = self.rotation.ccw();

        let kicks = &KICKS[self.shape as usize][(self.rotation as usize + 3) % 4];
        for (kick_col, kick_row) in kicks {
            let new = Piece {
                shape: self.shape,
                col: self.col - kick_col,
                row: self.row - kick_row,
                rotation: rotation,
            };

            if new.in_bounds() && !new.collides_in(board) {
                // println!("check col: {}, check row: {}", kick_col, kick_row);
                return new;
            }
        }

        self
    }
}

static PIECE_SHAPES: [[u64; 4]; 7] = [
    [
        // I
        0b1111,
        0b1000000000100000000010000000001,
        0b1111,
        0b1000000000100000000010000000001,
    ],
    [
        // J
        0b1_0000000111,
        0b11_0000000001_0000000001,
        0b111_0000000100,
        0b1_0000000001_00000000011,
    ],
    [
        // L
        0b100_0000000111,
        0b1_0000000001_0000000011,
        0b111_0000000001,
        0b11_0000000010_0000000010,
    ],
    [
        // O
        0b11_0000000011,
        0b11_0000000011,
        0b11_0000000011,
        0b11_0000000011,
    ],
    [
        // S
        0b110_0000000011,
        0b1_0000000011_0000000010,
        0b110_0000000011,
        0b1_0000000011_0000000010,
    ],
    [
        // T
        0b10_0000000111,
        0b1_0000000011_0000000001,
        0b111_0000000010,
        0b10_0000000011_0000000010,
    ],
    [
        // Z
        0b11_0000000110,
        0b10_0000000011_0000000001,
        0b110000000110,
        0b10_0000000011_0000000001,
    ],
];

static PIECE_MAX_COLS: [[i8; 4]; 7] = [
    [6, 9, 6, 9], /* I */
    [7, 8, 7, 8], /* J */
    [7, 8, 7, 8], /* L */
    [8, 8, 8, 8], /* O */
    [7, 8, 7, 8], /* S */
    [7, 8, 7, 8], /* T */
    [7, 8, 7, 8], /* Z */
];

static JLSTZ_KICKS: [[(i8, i8); 5]; 4] = [
    [(1, -1), (0, -1), (0, 0), (1, -3), (0, -3)],
    [(-1, 0), (0, 0), (0, -1), (-1, 2), (0, 2)],
    [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
    [(0, 1), (-1, 1), (-1, 0), (0, 3), (-1, 3)],
];

static I_KICKS: [[(i8, i8); 5]; 4] = [
    [(2, -2), (0, -2), (3, -2), (0, -3), (3, 0)],
    [(-2, 1), (-3, 1), (0, 1), (-3, 3), (0, 0)],
    [(1, -1), (3, -1), (0, -1), (3, 0), (0, -3)],
    [(-1, 2), (0, 2), (-3, 2), (0, 0), (-3, 3)],
];

static O_KICKS: [[(i8, i8); 5]; 4] = [[(0, 0); 5]; 4];

static KICKS: [&[[(i8, i8); 5]; 4]; 7] = [
    &I_KICKS,     /* I */
    &JLSTZ_KICKS, /* J */
    &JLSTZ_KICKS, /* L */
    &O_KICKS,     /* O */
    &JLSTZ_KICKS, /* S */
    &JLSTZ_KICKS, /* T */
    &JLSTZ_KICKS, /* Z */
];

pub const BOARD_MASK: u64 = 0b1111111111_1111111111_1111111111_1111111111;

impl Shape {
    pub fn name(self) -> char {
        ['I', 'J', 'L', 'O', 'S', 'T', 'Z'][self as usize]
    }

    pub fn from_ppt(val: u32) -> Shape {
        match val {
            0 => Shape::S,
            1 => Shape::Z,
            2 => Shape::J,
            3 => Shape::L,
            4 => Shape::T,
            5 => Shape::O,
            6 => Shape::I,
            _ => panic!("function \"from_ppt\" argument error."),
        }
    }
}

impl Rotation {
    pub fn cw(self) -> Rotation {
        match self {
            Rotation::None => Rotation::Clockwise,
            Rotation::Clockwise => Rotation::Half,
            Rotation::Half => Rotation::ConterClockwise,
            Rotation::ConterClockwise => Rotation::None,
        }
    }

    pub fn ccw(self) -> Rotation {
        match self {
            Rotation::Clockwise => Rotation::None,
            Rotation::Half => Rotation::Clockwise,
            Rotation::ConterClockwise => Rotation::Half,
            Rotation::None => Rotation::ConterClockwise,
        }
    }
}

// I use a 64-bit number to represent a queue of piece which is named of 'PieceQueue'. In the number, the lowest 3-bit
// number represents the first piece in 'next' pieces, the 2st lowest 3-bit number represents the second piece in 'next'
// pieces, and so on.

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PieceQueue(pub u64);

impl PieceQueue {
    pub fn new(queue: Vec<Shape>) -> PieceQueue {
        let mut res = 0;

        for ch in queue.iter().rev() {
            res <<= 3;
            res |= match ch {
                Shape::I => 0,
                Shape::J => 1,
                Shape::L => 2,
                Shape::O => 3,
                Shape::S => 4,
                Shape::T => 5,
                Shape::Z => 6,
            }
        }

        PieceQueue(res)
    }

    pub fn now(&self) -> Shape {
        match self.0 & 0b_111 {
            0 => Shape::I,
            1 => Shape::J,
            2 => Shape::L,
            3 => Shape::O,
            4 => Shape::S,
            5 => Shape::T,
            6 => Shape::Z,
            _ => panic!("function \"now\" argument error"),
        }
    }

    pub fn go(&self) -> PieceQueue {
        PieceQueue(self.0 >> 3)
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
}
