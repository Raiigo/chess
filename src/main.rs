#[derive(Debug, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    color: Color,
    piece_type: PieceType,
    castle: Option<bool>,
    jump: Option<bool>,
    en_passant: Option<bool>,
    check_move: fn([[Option<Piece>; 8]; 8], (usize, usize), (usize, usize)) -> bool,
}

impl Piece {
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

const PAWN: Piece = Piece {
    color: Color::White,
    piece_type: PieceType::Pawn,
    castle: None,
    jump: Some(true),
    en_passant: Some(false),
    check_move: |board: [[Option<Piece>; 8]; 8], start_pos: (usize, usize), end_pos: (usize, usize)| -> bool {
        false
    }
};
const ROOK: Piece = Piece {
    color: Color::White,
    piece_type: PieceType::Rook,
    castle: Some(true),
    jump: None,
    en_passant: None,
    check_move: |board: [[Option<Piece>; 8]; 8], start_pos: (usize, usize), end_pos: (usize, usize)| -> bool {
        false
    }
};
const KNIGHT: Piece = Piece {
    color: Color::White,
    piece_type: PieceType::Knight,
    castle: None,
    jump: None,
    en_passant: None,
    check_move: |board: [[Option<Piece>; 8]; 8], start_pos: (usize, usize), end_pos: (usize, usize)| -> bool {
        false
    }
};
const BISHOP: Piece = Piece {
    color: Color::White,
    piece_type: PieceType::Bishop,
    castle: None,
    jump: None,
    en_passant: None,
    check_move: |board: [[Option<Piece>; 8]; 8], start_pos: (usize, usize), end_pos: (usize, usize)| -> bool {
        false
    }
};
const QUEEN: Piece = Piece {
    color: Color::White,
    piece_type: PieceType::Queen,
    castle: None,
    jump: None,
    en_passant: None,
    check_move: |board: [[Option<Piece>; 8]; 8], start_pos: (usize, usize), end_pos: (usize, usize)| -> bool {
        false
    }
};
const KING: Piece = Piece {
    color: Color::White,
    piece_type: PieceType::King,
    castle: Some(true),
    jump: None,
    en_passant: None,
    check_move: |board: [[Option<Piece>; 8]; 8], start_pos: (usize, usize), end_pos: (usize, usize)| -> bool {
        false
    }
};

fn main() {

    let mut board: [[Option<Piece>; 8]; 8] = [[Some(ROOK.clone().color(Color::White)), Some(PAWN.clone().color(Color::White)), None, None, None, None, Some(PAWN.clone().color(Color::Black)), Some(ROOK.clone().color(Color::Black))],
                                              [Some(KNIGHT.clone().color(Color::White)), Some(PAWN.clone().color(Color::White)), None, None, None, None, Some(PAWN.clone().color(Color::Black)), Some(KNIGHT.clone().color(Color::Black))],
                                              [Some(BISHOP.clone().color(Color::White)), Some(PAWN.clone().color(Color::White)), None, None, None, None, Some(PAWN.clone().color(Color::Black)), Some(BISHOP.clone().color(Color::Black))],
                                              [Some(QUEEN.clone().color(Color::White)), Some(PAWN.clone().color(Color::White)), None, None, None, None, Some(PAWN.clone().color(Color::Black)), Some(QUEEN.clone().color(Color::Black))],
                                              [Some(KING.clone().color(Color::White)), Some(PAWN.clone().color(Color::White)), None, None, None, None, Some(PAWN.clone().color(Color::Black)), Some(KING.clone().color(Color::Black))],
                                              [Some(BISHOP.clone().color(Color::White)), Some(PAWN.clone().color(Color::White)), None, None, None, None, Some(PAWN.clone().color(Color::Black)), Some(BISHOP.clone().color(Color::Black))],
                                              [Some(KNIGHT.clone().color(Color::White)), Some(PAWN.clone().color(Color::White)), None, None, None, None, Some(PAWN.clone().color(Color::Black)), Some(KNIGHT.clone().color(Color::Black))],
                                              [Some(ROOK.clone().color(Color::White)), Some(PAWN.clone().color(Color::White)), None, None, None, None, Some(PAWN.clone().color(Color::Black)), Some(ROOK.clone().color(Color::Black))]];

    dbg!(board);

}