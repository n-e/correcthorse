use derive_more::{BitAnd, BitOr};
use std::fmt::Debug;
use std::ops::{Index, IndexMut, Not};
use std::sync::LazyLock;

use crate::util::Ternary::{self, No};

const DIR_STRAIGHT: [(i8, i8); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const DIR_DIAG: [(i8, i8); 4] = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
const DIR_KNIGHT: [(i8, i8); 8] = [
    (2, 1),
    (2, -1),
    (-2, 1),
    (-2, -1),
    (-1, 2),
    (1, 2),
    (-1, -2),
    (1, -2),
];
const DIR_KING: LazyLock<Vec<(i8, i8)>> = LazyLock::new(|| [DIR_DIAG, DIR_STRAIGHT].concat());
const SLIDERS_DIRS: LazyLock<Vec<(Piece, i8, i8)>> = LazyLock::new(|| {
    [
        (Piece::Queen, DIR_STRAIGHT),
        (Piece::Queen, DIR_DIAG),
        (Piece::Rook, DIR_STRAIGHT),
        (Piece::Bishop, DIR_DIAG),
    ]
    .iter()
    .flat_map(|(piece, dirs)| dirs.map(|(x, y)| (*piece, x, y)))
    .collect()
});

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Piece {
    King = 0,
    Queen = 1,
    Rook = 2,
    Bishop = 3,
    Knight = 4,
    Pawn = 5,
}

impl Piece {
    pub fn from_usize(value: usize) -> Option<Self> {
        match value {
            0 => Some(Self::King),
            1 => Some(Self::Queen),
            2 => Some(Self::Rook),
            3 => Some(Self::Bishop),
            4 => Some(Self::Knight),
            5 => Some(Self::Pawn),
            _ => None,
        }
    }

    pub fn from_char(value: char) -> Option<Self> {
        match value {
            'k' => Some(Self::King),
            'q' => Some(Self::Queen),
            'r' => Some(Self::Rook),
            'b' => Some(Self::Bishop),
            'n' => Some(Self::Knight),
            'p' => Some(Self::Pawn),
            _ => None,
        }
    }
    pub fn to_str(&self) -> String {
        let v = match self {
            Piece::King => "k",
            Piece::Queen => "q",
            Piece::Rook => "r",
            Piece::Bishop => "b",
            Piece::Knight => "n",
            Piece::Pawn => "p",
        };
        v.to_string()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn is_white(&self) -> bool {
        return *self == Color::White;
    }

    pub fn from_white_hot(is_white: bool) -> Self {
        return if is_white { Color::White } else { Color::Black };
    }

    pub fn other(&self) -> Self {
        match *self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub struct Square(i8);

impl Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.to_algebraic())
    }
}

impl Square {
    pub fn all() -> impl Iterator<Item = Self> {
        (0..64).map(|i| Self(i))
    }

    pub fn from_algebraic(alg: String) -> Self {
        let chr = alg.as_bytes();

        Self::from_coords(chr[0] as i8 - 'a' as i8, chr[1] as i8 - '1' as i8)
    }

    pub fn to_algebraic(&self) -> String {
        let file = self.file();
        let rank = self.rank();

        let letter = ('a' as u8 + file as u8) as char;

        format!("{}{}", letter, rank + 1)
    }

    pub fn file(&self) -> i8 {
        self.0 % 8
    }
    pub fn rank(&self) -> i8 {
        (self.0 - self.0 % 8) / 8
    }

    pub fn from_coords(file: i8, rank: i8) -> Self {
        Self(rank * 8 + file)
    }

    pub fn offset(&self, files: i8, ranks: i8) -> Option<Self> {
        let file = self.file() + files;
        let rank = self.rank() + ranks;
        if file > 7 || file < 0 || rank > 7 || rank < 0 {
            None
        } else {
            Some(Self::from_coords(file, rank))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Move {
    from: Square,
    to: Square,
    promo: Option<Piece>,
}

impl Move {
    pub fn to_lan(&self) -> String {
        format!(
            "{}{}{}",
            self.from.to_algebraic(),
            self.to.to_algebraic(),
            self.promo.map_or("".to_string(), |x| x.to_str()),
        )
    }
    pub fn from_lan(lan: String) -> Self {
        let from = &lan[0..2];
        let to = &lan[2..4];
        let promo = &lan[4..];

        Move {
            from: Square::from_algebraic(from.to_string()),
            to: Square::from_algebraic(to.to_string()),
            promo: promo
                .chars()
                .nth(0)
                .map(|chr| Piece::from_char(chr).unwrap()),
        }
    }
}

#[derive(BitAnd, BitOr, Clone, Copy)]
pub struct BitBoard(i64);

impl Not for BitBoard {
    type Output = BitBoard;

    fn not(self) -> Self::Output {
        BitBoard(!self.0)
    }
}

impl Debug for BitBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("BitBoard").field(&self.0).finish()
    }
}

impl BitBoard {
    const EMPTY: Self = Self(0);

    // python3 -c 'print("\n".join(f"pub const {chr(65+c)}{r}: Self = Self(1 << {(r-1)*8+c});" for r in range(1,9) for c in range(8)))'|pbcopy
    pub const A1: Self = Self(1 << 0);
    pub const B1: Self = Self(1 << 1);
    pub const C1: Self = Self(1 << 2);
    pub const D1: Self = Self(1 << 3);
    pub const E1: Self = Self(1 << 4);
    pub const F1: Self = Self(1 << 5);
    pub const G1: Self = Self(1 << 6);
    pub const H1: Self = Self(1 << 7);
    pub const A2: Self = Self(1 << 8);
    pub const B2: Self = Self(1 << 9);
    pub const C2: Self = Self(1 << 10);
    pub const D2: Self = Self(1 << 11);
    pub const E2: Self = Self(1 << 12);
    pub const F2: Self = Self(1 << 13);
    pub const G2: Self = Self(1 << 14);
    pub const H2: Self = Self(1 << 15);
    pub const A3: Self = Self(1 << 16);
    pub const B3: Self = Self(1 << 17);
    pub const C3: Self = Self(1 << 18);
    pub const D3: Self = Self(1 << 19);
    pub const E3: Self = Self(1 << 20);
    pub const F3: Self = Self(1 << 21);
    pub const G3: Self = Self(1 << 22);
    pub const H3: Self = Self(1 << 23);
    pub const A4: Self = Self(1 << 24);
    pub const B4: Self = Self(1 << 25);
    pub const C4: Self = Self(1 << 26);
    pub const D4: Self = Self(1 << 27);
    pub const E4: Self = Self(1 << 28);
    pub const F4: Self = Self(1 << 29);
    pub const G4: Self = Self(1 << 30);
    pub const H4: Self = Self(1 << 31);
    pub const A5: Self = Self(1 << 32);
    pub const B5: Self = Self(1 << 33);
    pub const C5: Self = Self(1 << 34);
    pub const D5: Self = Self(1 << 35);
    pub const E5: Self = Self(1 << 36);
    pub const F5: Self = Self(1 << 37);
    pub const G5: Self = Self(1 << 38);
    pub const H5: Self = Self(1 << 39);
    pub const A6: Self = Self(1 << 40);
    pub const B6: Self = Self(1 << 41);
    pub const C6: Self = Self(1 << 42);
    pub const D6: Self = Self(1 << 43);
    pub const E6: Self = Self(1 << 44);
    pub const F6: Self = Self(1 << 45);
    pub const G6: Self = Self(1 << 46);
    pub const H6: Self = Self(1 << 47);
    pub const A7: Self = Self(1 << 48);
    pub const B7: Self = Self(1 << 49);
    pub const C7: Self = Self(1 << 50);
    pub const D7: Self = Self(1 << 51);
    pub const E7: Self = Self(1 << 52);
    pub const F7: Self = Self(1 << 53);
    pub const G7: Self = Self(1 << 54);
    pub const H7: Self = Self(1 << 55);
    pub const A8: Self = Self(1 << 56);
    pub const B8: Self = Self(1 << 57);
    pub const C8: Self = Self(1 << 58);
    pub const D8: Self = Self(1 << 59);
    pub const E8: Self = Self(1 << 60);
    pub const F8: Self = Self(1 << 61);
    pub const G8: Self = Self(1 << 62);
    pub const H8: Self = Self(1 << 63);

    pub fn from_rank(i: i8) -> Self {
        Self(0xFF << i * 8)
    }

    pub fn from_file(i: i8) -> Self {
        Self(
            (Self::A1.0
                & Self::A2.0
                & Self::A3.0
                & Self::A4.0
                & Self::A5.0
                & Self::A6.0
                & Self::A7.0
                & Self::A8.0)
                << i,
        )
    }

    pub fn from_square(square: Square) -> Self {
        Self(1 << square.0)
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn hot_at(self, s: Square) -> bool {
        !(self & BitBoard::from_square(s)).is_empty()
    }

    pub fn set(&mut self, square: Square, value: bool) {
        if value {
            *self = *self | BitBoard::from_square(square);
        } else {
            *self = *self & !BitBoard::from_square(square);
        }
    }
}

#[derive(Clone)]
pub struct Board {
    pieces: [BitBoard; 6],
    white: BitBoard,
}

impl Index<Piece> for Board {
    type Output = BitBoard;

    fn index(&self, piece: Piece) -> &Self::Output {
        &self.pieces[piece as usize]
    }
}

impl IndexMut<Piece> for Board {
    fn index_mut(&mut self, piece: Piece) -> &mut Self::Output {
        &mut self.pieces[piece as usize]
    }
}

impl Board {
    pub fn start_pos() -> Self {
        Self {
            pieces: [
                BitBoard::E1 | BitBoard::E8,
                BitBoard::D1 | BitBoard::D8,
                BitBoard::A1 | BitBoard::A8 | BitBoard::H1 | BitBoard::H8,
                BitBoard::C1 | BitBoard::C8 | BitBoard::F1 | BitBoard::F8,
                BitBoard::B1 | BitBoard::B8 | BitBoard::G1 | BitBoard::G8,
                BitBoard::from_rank(1) | BitBoard::from_rank(6),
            ],
            white: BitBoard::from_rank(0) | BitBoard::from_rank(1),
        }
    }

    pub fn empty() -> Self {
        Self {
            pieces: [
                BitBoard::EMPTY,
                BitBoard::EMPTY,
                BitBoard::EMPTY,
                BitBoard::EMPTY,
                BitBoard::EMPTY,
                BitBoard::EMPTY,
            ],
            white: BitBoard::EMPTY,
        }
    }

    pub fn from_epd_part(rep: String) -> Self {
        let mut board = Self::empty();

        let mut rank = 7;
        let mut file = 0;
        for char in rep.chars() {
            if char == '/' {
                rank -= 1;
                file = 0;
            } else if char >= '0' && char <= '9' {
                file += char as i8 - '0' as i8;
            } else {
                let piece = Piece::from_char(char.to_ascii_lowercase()).unwrap();
                let square = Square::from_coords(file, rank);
                board[piece].set(square, true);
                board.white.set(square, char.is_ascii_uppercase());

                file += 1;
            }
        }

        board
    }

    pub fn piece_at(&self, square: Square) -> Option<Piece> {
        let piece_idx = self.pieces.iter().position(|x| x.hot_at(square));
        piece_idx.map(|idx| Piece::from_usize(idx).unwrap())
    }

    pub fn is_piece_at(&self, piece: Piece, square: Square) -> bool {
        self[piece].hot_at(square)
    }

    pub fn color_at(&self, square: Square) -> Color {
        Color::from_white_hot(self.white.hot_at(square))
    }

    pub fn is_color(&self, color: Color, square: Square) -> bool {
        self.white.hot_at(square) == color.is_white()
    }

    pub fn iterate_squares(&self) -> impl Iterator<Item = Option<(Square, Piece, Color)>> {
        Square::all().map(|square| {
            self.piece_at(square).map(|piece| {
                (
                    square,
                    piece,
                    Color::from_white_hot(self.white.hot_at(square)),
                )
            })
        })
    }

    pub fn play(&mut self, mov: &Move) {
        let piece = self.piece_at(mov.from).unwrap();
        let is_white = self.white.hot_at(mov.from);

        // En Passant
        if piece == Piece::Pawn
            && mov.from.file() != mov.to.file()
            && self.piece_at(mov.to).is_none()
        {
            let pawn_to_remove = mov.to.offset(0, if is_white { -1 } else { 1 }).unwrap();
            self[piece].set(pawn_to_remove, false);
            self.white.set(pawn_to_remove, false);
        }

        for p in [
            Piece::King,
            Piece::Queen,
            Piece::Rook,
            Piece::Bishop,
            Piece::Knight,
            Piece::Pawn,
        ] {
            self[p].set(mov.to, false);
        }

        self[piece].set(mov.from, false);
        self[mov.promo.unwrap_or(piece)].set(mov.to, true);
        self.white.set(mov.from, false);
        self.white.set(mov.to, is_white);

        if piece == Piece::King {
            let vec = mov.to.file() - mov.from.file();

            if vec == 2 || vec == -2 {
                let rook_rank = mov.from.rank();
                let rook_from_file: i8 = if vec == 2 { 7 } else { 0 };
                let rook_from = Square::from_coords(rook_from_file, rook_rank);

                let rook_to = if vec == 2 {
                    mov.to.offset(-1, 0).unwrap()
                } else {
                    mov.to.offset(1, 0).unwrap()
                };

                self[Piece::Rook].set(rook_from, false);
                self[Piece::Rook].set(rook_to, true);
                self.white.set(rook_from, false);
                self.white.set(rook_to, is_white);
            }
        }
    }

    fn has_attacking_slider(
        &self,
        from: Square,
        piece: Piece,
        color: Color,
        dir_x: i8,
        dir_y: i8,
    ) -> bool {
        for i in 1..8 {
            let square = from.offset(dir_x * i, dir_y * i);

            match square {
                Some(square) => {
                    if let Some(tgt_piece) = self.piece_at(square) {
                        if tgt_piece == piece && self.is_color(color, square) {
                            // println!("attacker: {:?} {:?}", tgt_piece, square);
                            return true;
                        }
                        return false;
                    }
                }
                None => return false,
            }
        }

        false
    }

    pub fn is_attacked(&self, square: Square, attacking_color: Color) -> bool {
        // This is reversed, as we go from the attacked piece to the pawn
        let dir_pawn = match attacking_color {
            Color::White => [(-1, -1), (1, -1)],
            Color::Black => [(-1, 1), (1, 1)],
        };

        let attacked_by_sl = SLIDERS_DIRS.iter().any(|(piece, dir_x, dir_y)| {
            self.has_attacking_slider(square, *piece, attacking_color, *dir_x, *dir_y)
        });
        let attacked_by_king = DIR_KING.iter().any(|(x, y)| {
            square.offset(*x, *y).map_or(false, |s| {
                self.is_piece_at(Piece::King, s) && self.is_color(attacking_color, s)
            })
        });
        let attacked_by_knight = DIR_KNIGHT.iter().any(|(x, y)| {
            square.offset(*x, *y).map_or(false, |s| {
                self.is_piece_at(Piece::Knight, s) && self.is_color(attacking_color, s)
            })
        });
        let attacked_by_pawn = dir_pawn.iter().any(|(x, y)| {
            square.offset(*x, *y).map_or(false, |s| {
                self.is_piece_at(Piece::Pawn, s) && self.is_color(attacking_color, s)
            })
        });

        let attacked = attacked_by_sl || attacked_by_king || attacked_by_knight || attacked_by_pawn;

        // if square == Square::from_coords(7, 0)
        //     && self
        //         .piece_at(Square::from_algebraic("g2".to_string()))
        //         .is_some()
        // {
        //     println!("yo {} {:?}", attacked_by_pawn, dir_pawn)
        // }

        attacked
    }

    fn pseudo_legal_moves_by_directions(
        &self,
        directions: Vec<(i8, i8)>,
        max_steps: i8,
        capture_mode: &Ternary,
        from: Square,
        side: Color,
    ) -> Vec<Move> {
        directions
            .iter()
            .flat_map(move |(x, y)| {
                let mut moves = vec![];
                for i in 1..max_steps + 1 {
                    let maybe_to = from.offset(*x * i, *y * i);

                    if let Some(to) = maybe_to {
                        let p = self.piece_at(to);
                        let c = self.color_at(to);

                        // if from == Square::from_coords(2, 0) && to == Square::from_coords(6, 4) {
                        //     eprintln!("yo {:?} {:?}", p, c);
                        // }

                        // go to empty square or capture
                        if match capture_mode {
                            Ternary::No => p.is_none(),
                            Ternary::Both => p.is_none() || c == side.other(),
                            Ternary::Yes => !p.is_none() && c == side.other(),
                        } {
                            moves.push(Move {
                                from: from,
                                to: to,
                                promo: None,
                            })
                        }

                        if p.is_some() {
                            break;
                        }
                    }
                }

                moves
            })
            .collect()
    }

    pub fn pseudo_legal_moves(
        &self,
        side: Color,
        castling_rights: (bool, bool),
        ep_square: Option<Square>,
    ) -> Vec<Move> {
        self.iterate_squares()
            .flat_map(|x| match x {
                Some((from, piece, color)) => {
                    if color != side {
                        vec![]
                    } else if piece == Piece::Queen
                        || piece == Piece::Rook
                        || piece == Piece::Bishop
                    {
                        let directions = SLIDERS_DIRS
                            .iter()
                            .filter(move |(p, _, _)| piece == *p)
                            .map(|(_, x, y)| (*x, *y))
                            .collect();

                        self.pseudo_legal_moves_by_directions(
                            directions,
                            7,
                            &Ternary::Both,
                            from,
                            side,
                        )
                    } else if piece == Piece::King {
                        let mut king_moves = self.pseudo_legal_moves_by_directions(
                            DIR_KING.to_vec(),
                            1,
                            &Ternary::Both,
                            from,
                            side,
                        );

                        // println!("castling {:?}", castling_rights);

                        if castling_rights.0
                            && self.piece_at(from.offset(-1, 0).unwrap()).is_none()
                            && self.piece_at(from.offset(-2, 0).unwrap()).is_none()
                            && self.piece_at(from.offset(-3, 0).unwrap()).is_none()
                            && !self.is_attacked(from, side.other())
                            && !self.is_attacked(from.offset(-1, 0).unwrap(), side.other())
                        {
                            king_moves.push(Move {
                                from: from,
                                to: from.offset(-2, 0).unwrap(),
                                promo: None,
                            });
                        }
                        if castling_rights.1
                            && self.piece_at(from.offset(1, 0).unwrap()).is_none()
                            && self.piece_at(from.offset(2, 0).unwrap()).is_none()
                            && !self.is_attacked(from, side.other())
                            && !self.is_attacked(from.offset(1, 0).unwrap(), side.other())
                        {
                            king_moves.push(Move {
                                from: from,
                                to: from.offset(2, 0).unwrap(),
                                promo: None,
                            });
                        }
                        // println!("{:?}", king_moves);
                        king_moves
                    } else if piece == Piece::Knight {
                        self.pseudo_legal_moves_by_directions(
                            DIR_KNIGHT.to_vec(),
                            1,
                            &Ternary::Both,
                            from,
                            side,
                        )
                    } else if piece == Piece::Pawn {
                        let dir_y = match side {
                            Color::White => 1,
                            Color::Black => -1,
                        };
                        let is_first_rank = match side {
                            Color::White => from.rank() == 1,
                            Color::Black => from.rank() == 6,
                        };
                        let is_promo_rank = match side {
                            Color::White => from.rank() == 6,
                            Color::Black => from.rank() == 1,
                        };

                        let moves = self.pseudo_legal_moves_by_directions(
                            vec![(0, dir_y)],
                            if is_first_rank { 2 } else { 1 },
                            &Ternary::No,
                            from,
                            side,
                        );

                        let captures = self.pseudo_legal_moves_by_directions(
                            vec![(1, dir_y), (-1, dir_y)],
                            1,
                            &Ternary::Yes,
                            from,
                            side,
                        );

                        let mut en_passant = vec![];
                        if let Some(ep_square) = ep_square {
                            if Some(ep_square) == from.offset(1, dir_y)
                                || Some(ep_square) == from.offset(-1, dir_y)
                            {
                                en_passant.push(Move {
                                    from: from,
                                    to: ep_square,
                                    promo: None,
                                })
                            }
                        }

                        if is_promo_rank {
                            [moves, captures, en_passant]
                                .concat()
                                .iter()
                                .flat_map(|m| {
                                    [Piece::Queen, Piece::Rook, Piece::Bishop, Piece::Knight].map(
                                        |p| Move {
                                            from: m.from,
                                            to: m.to,
                                            promo: Some(p),
                                        },
                                    )
                                })
                                .collect()
                        } else {
                            [moves, captures, en_passant].concat()
                        }
                    } else {
                        vec![]
                    }
                }
                None => vec![],
            })
            .collect()
    }

    pub fn is_in_check(&self, color: Color) -> bool {
        // TODO : slow
        let king = self
            .iterate_squares()
            .find(|s| s.is_some_and(|(_, p, c)| p == Piece::King && c == color))
            .unwrap()
            .unwrap();

        self.is_attacked(king.0, color.other())
    }
}

#[derive(Clone)]
pub struct Position {
    board: Board,
    side: Color,
    white_castling_rights: (bool, bool),
    black_castling_rights: (bool, bool),
    ep_square: Option<Square>,
}

impl Position {
    pub fn start_pos() -> Position {
        Position {
            board: Board::start_pos(),
            side: Color::White,
            white_castling_rights: (true, true),
            black_castling_rights: (true, true),
            ep_square: None,
        }
    }

    pub fn from_epd(rep: String) -> Self {
        let parts: Vec<_> = rep.split(" ").collect();
        assert_eq!(parts.len(), 4);
        let board = parts[0];
        let side = parts[1];
        let castling = parts[2];
        let ep = parts[3];

        Self {
            board: Board::from_epd_part(board.into()),
            side: Color::from_white_hot(side == "w"),
            white_castling_rights: (castling.contains("Q"), castling.contains("K")),
            black_castling_rights: (castling.contains("q"), castling.contains("k")),
            ep_square: if ep == "-" {
                None
            } else {
                Some(Square::from_algebraic(ep.to_string()))
            },
        }
    }

    pub fn pseudo_legal_moves(&self) -> Vec<Move> {
        self.board.pseudo_legal_moves(
            self.side,
            match self.side {
                Color::White => self.white_castling_rights,
                Color::Black => self.black_castling_rights,
            },
            self.ep_square,
        )
    }

    // Verify that the king of the side that moved is not in check
    pub fn is_fully_legal(&self) -> bool {
        !self.board.is_in_check(self.side.other())
    }

    pub fn play(&mut self, mov: &Move) {
        self.board.play(mov);
        self.side = self.side.other();

        // EN PASSANT
        if self.board.piece_at(mov.to) == Some(Piece::Pawn)
            && (mov.to.rank() - mov.from.rank()).abs() == 2
        {
            self.ep_square = mov.from.offset(
                0,
                if self.side.other() == Color::White {
                    1
                } else {
                    -1
                },
            );
        } else {
            self.ep_square = None;
        }

        // CASTLING
        let vec = mov.to.file() - mov.from.file();
        let is_castling =
            self.board.piece_at(mov.to) == Some(Piece::King) && (vec == 2 || vec == -2);
        if is_castling {
            match self.side.other() {
                Color::White => self.white_castling_rights = (false, false),
                Color::Black => self.black_castling_rights = (false, false),
            }
        }

        // no castling if the rook was moved or was captured
        if [mov.from, mov.to].contains(&Square::from_coords(0, 0)) {
            self.white_castling_rights.0 = false;
        }
        if [mov.from, mov.to].contains(&Square::from_coords(7, 0)) {
            self.white_castling_rights.1 = false;
        }
        if [mov.from, mov.to].contains(&Square::from_coords(0, 7)) {
            self.black_castling_rights.0 = false;
        }
        if [mov.from, mov.to].contains(&Square::from_coords(7, 7)) {
            self.black_castling_rights.1 = false;
        }
        // no castling if the king has moved
        if mov.from == Square::from_coords(4, 0) {
            self.white_castling_rights.0 = false;
            self.white_castling_rights.1 = false;
        }
        if mov.from == Square::from_coords(4, 7) {
            self.black_castling_rights.0 = false;
            self.black_castling_rights.1 = false;
        }
    }

    pub fn undo(&self) {}
}
