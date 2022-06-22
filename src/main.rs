#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Clone, Copy)]
pub struct Piece {
    color: Color,
    piece_type: PieceType,
    castle: Option<bool>,
    jump: Option<bool>,
    en_passant: Option<bool>,
    check_move: fn(&[[Option<Piece>; 8]; 8], (usize, usize), (usize, usize)) -> bool,
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
    check_move: |board: &[[Option<Piece>; 8]; 8], start_pos: (usize, usize), end_pos: (usize, usize)| -> bool {
        let move_vec: (isize, isize) = ((end_pos.0 - start_pos.0) as isize, (end_pos.1 - start_pos.1) as isize);
        let piece = match board[start_pos.0][start_pos.1] {
            Some(piece) => if piece.piece_type != PieceType::Pawn {
                return false;
            } else {
                piece
            },
            None => return false, // No piece has been selected to be moved
        };
        let end_opt = board[end_pos.0][end_pos.1];

        match piece.color {
            Color::White => {
                // Check if both direction are in range
                if move_vec.1 < 1 || move_vec.1 > 2 || move_vec.0 < -1 || move_vec.0 > 1 {
                    return false;
                }
                if move_vec.1 == 1 {
                    // Check if diagonal take is possible
                    if move_vec.0 == -1 || move_vec.0 == 1 { // Here we have two cases: taking directly an enemy piece or en passant capture
                        match end_opt {
                            Some(end_piece) => if end_piece.color == Color::Black { // Check standard capture
                                return true;
                            } else {
                                return false;
                            },
                            None => {
                                let en_passant_piece_opt = board[end_pos.0][end_pos.1 - 1];
                                match en_passant_piece_opt {
                                    Some(en_passant_piece) => {
                                        if en_passant_piece.piece_type == PieceType::Pawn {
                                            if en_passant_piece.en_passant.unwrap() && en_passant_piece.color == Color::Black {
                                                return true;
                                            } else {
                                                return false;
                                            }
                                        } else {
                                            return false;
                                        }
                                    },
                                    None => return false, // No piece for a en passant capture
                                }
                            },
                        }
                    } else {
                        match end_opt {
                            Some(_) => return false,
                            None => return true,
                        }
                    }
                } else if move_vec.1 == 2 && piece.jump.unwrap() {
                    match end_opt {
                        Some(_) => return false,
                        None => {
                            match board[end_pos.0][end_pos.1 - 1] {
                                Some(_) => return false,
                                None => return true,
                            }
                        },
                    }
                } else {
                    return false;
                }
            },
            Color::Black => {
                // Check if both direction are in range
                if move_vec.1 > -1 || move_vec.1 < -2 || move_vec.0 < -1 || move_vec.0 > 1 {
                    return false;
                }
                if move_vec.1 == -1 {
                    // Check if diagonal take is possible
                    if move_vec.0 == -1 || move_vec.0 == 1 { // Here we have two cases: taking directly an enemy piece or en passant capture
                        match end_opt {
                            Some(end_piece) => if end_piece.color == Color::White { // Check standard capture
                                return true;
                            } else {
                                return false;
                            },
                            None => {
                                let en_passant_piece_opt = board[end_pos.0][end_pos.1 + 1];
                                match en_passant_piece_opt {
                                    Some(en_passant_piece) => {
                                        if en_passant_piece.piece_type == PieceType::Pawn {
                                            if en_passant_piece.en_passant.unwrap() && en_passant_piece.color == Color::White {
                                                return true;
                                            } else {
                                                return false;
                                            }
                                        } else {
                                            return false;
                                        }
                                    },
                                    None => return false, // No piece for a en passant capture
                                }
                            },
                        }
                    } else {
                        match end_opt {
                            Some(_) => return false,
                            None => return true,
                        }
                    }
                } else if move_vec.1 == 2 && piece.jump.unwrap() {
                    match end_opt {
                        Some(_) => return false,
                        None => {
                            match board[end_pos.0][end_pos.1 + 1] {
                                Some(_) => return false,
                                None => return true,
                            }
                        },
                    }
                } else {
                    return false;
                }
            },
        }
    }
};
const ROOK: Piece = Piece {
    color: Color::White,
    piece_type: PieceType::Rook,
    castle: Some(true),
    jump: None,
    en_passant: None,
    check_move: |board: &[[Option<Piece>; 8]; 8], start_pos: (usize, usize), end_pos: (usize, usize)| -> bool {
        let move_vec: (isize, isize) = ((end_pos.0 - start_pos.0) as isize, (end_pos.1 - start_pos.1) as isize);
        let piece = match board[start_pos.0][start_pos.1] {
            Some(piece) => piece,
            None => return false, // No piece has been selected to be moved
        };
        let end_opt = board[end_pos.0][end_pos.1];

        // Check directions
        if (move_vec.0 != 0 && move_vec.1 != 0) || (move_vec.0 == 0 && move_vec.1 == 0) {
            return false;
        }

        let mut pos_set: Vec<Option<Piece>> = vec![];

        if move_vec.0 != 0 {
            if move_vec.0 < 0 {
                for i in end_pos.0..start_pos.0 {
                    pos_set.push(board[i][start_pos.1]);
                }
            } else if move_vec.0 > 0 {
                for i in (start_pos.0 + 1)..(end_pos.0 + 1) {
                    pos_set.push(board[i][start_pos.1]);
                }
            }
        } else if move_vec.1 != 0 {
            if move_vec.1 < 0 {
                for i in end_pos.1..start_pos.1 {
                    pos_set.push(board[start_pos.0][i]);
                }
            } else if move_vec.1 > 0 {
                for i in (start_pos.1 + 1)..(end_pos.1 + 1) {
                    pos_set.push(board[start_pos.0][i]);
                }
            }
        } else {
            return false;
        }

        for i in 0..(pos_set.len() - 1) {
            match pos_set[i] {
                Some(_) => return false,
                None => continue,
            }
        }

        match piece.color {
            Color::White => {
                match end_opt {
                    Some(end_piece) => match end_piece.color {
                        Color::White => return false,
                        Color::Black => return true,
                    },
                    None => return true,
                }
            },
            Color::Black => {
                match end_opt {
                    Some(end_piece) => match end_piece.color {
                        Color::White => return true,
                        Color::Black => return false,
                    },
                    None => return true,
                }
            },
        }
    }
};
const KNIGHT: Piece = Piece {
    color: Color::White,
    piece_type: PieceType::Knight,
    castle: None,
    jump: None,
    en_passant: None,
    check_move: |board: &[[Option<Piece>; 8]; 8], start_pos: (usize, usize), end_pos: (usize, usize)| -> bool {
        let move_vec: (isize, isize) = ((end_pos.0 - start_pos.0) as isize, (end_pos.1 - start_pos.1) as isize);
        let piece = match board[start_pos.0][start_pos.1] {
            Some(piece) => if piece.piece_type != PieceType::Knight {
                return false;
            } else {
                piece
            },
            None => return false, // No piece has been selected to be moved
        };
        let end_opt = board[end_pos.0][end_pos.1];

        // Check directions
        if !(((move_vec.1 == 2 || move_vec.1 == -2) && (move_vec.0 == 1 || move_vec.0 == -1)) || ((move_vec.0 == 2 || move_vec.0 == -2) && (move_vec.1 == 1 || move_vec.1 == -1))) {
            return false;
        }

        match piece.color {
            Color::White => {
                match end_opt {
                    Some(end_piece) => match end_piece.color {
                        Color::White => return false,
                        Color::Black => return true,
                    },
                    None => return true,
                }
            },
            Color::Black => {
                match end_opt {
                    Some(end_piece) => match end_piece.color {
                        Color::White => return true,
                        Color::Black => return false,
                    },
                    None => return true,
                }
            },
        }
    }
};
const BISHOP: Piece = Piece {
    color: Color::White,
    piece_type: PieceType::Bishop,
    castle: None,
    jump: None,
    en_passant: None,
    check_move: |board: &[[Option<Piece>; 8]; 8], start_pos: (usize, usize), end_pos: (usize, usize)| -> bool {
        let move_vec: (isize, isize) = ((end_pos.0 - start_pos.0) as isize, (end_pos.1 - start_pos.1) as isize);
        let piece = match board[start_pos.0][start_pos.1] {
            Some(piece) => piece,
            None => return false, // No piece has been selected to be moved
        };
        let end_opt = board[end_pos.0][end_pos.1];

        if move_vec.0.abs() != move_vec.1.abs() || move_vec.0 == 0 {
            return false;
        }

        let mut pos_set: Vec<Option<Piece>> = vec![];

        if move_vec.0 < 0 {
            if move_vec.1 < 0 {
                for i in end_pos.0..start_pos.0 {
                    for j in end_pos.1..start_pos.1 {
                        pos_set.push(board[i][j]);
                    }
                }
            } else if move_vec.1 > 0 {
                for i in end_pos.0..start_pos.0 {
                    for j in (start_pos.1 + 1)..(end_pos.1 + 1) {
                        pos_set.push(board[i][j]);
                    }
                }
            }
        } else if move_vec.0 > 0 {
            if move_vec.1 < 0 {
                for i in (start_pos.0 + 1)..(end_pos.0 + 1) {
                    for j in end_pos.1..start_pos.1 {
                        pos_set.push(board[i][j]);
                    }
                }
            } else if move_vec.1 > 0 {
                for i in (start_pos.0 + 1)..(end_pos.0 + 1) {
                    for j in (start_pos.1 + 1)..(end_pos.1 + 1) {
                        pos_set.push(board[i][j]);
                    }
                }
            }
        }

        for i in 0..(pos_set.len() - 1) {
            match pos_set[i] {
                Some(_) => return false,
                None => continue,
            }
        }

        match piece.color {
            Color::White => {
                match end_opt {
                    Some(end_piece) => match end_piece.color {
                        Color::White => return false,
                        Color::Black => return true,
                    },
                    None => return true,
                }
            },
            Color::Black => {
                match end_opt {
                    Some(end_piece) => match end_piece.color {
                        Color::White => return true,
                        Color::Black => return false,
                    },
                    None => return true,
                }
            },
        }
    }
};
const QUEEN: Piece = Piece {
    color: Color::White,
    piece_type: PieceType::Queen,
    castle: None,
    jump: None,
    en_passant: None,
    check_move: |board: &[[Option<Piece>; 8]; 8], start_pos: (usize, usize), end_pos: (usize, usize)| -> bool {
        if (ROOK.check_move)(board, start_pos, end_pos) || (BISHOP.check_move)(board, start_pos, end_pos) {
            return true
        } else {
            false
        }
    }
};
const KING: Piece = Piece {
    color: Color::White,
    piece_type: PieceType::King,
    castle: Some(true),
    jump: None,
    en_passant: None,
    check_move: |board: &[[Option<Piece>; 8]; 8], start_pos: (usize, usize), end_pos: (usize, usize)| -> bool {
        let move_vec: (isize, isize) = ((end_pos.0 - start_pos.0) as isize, (end_pos.1 - start_pos.1) as isize);
        let piece = match board[start_pos.0][start_pos.1] {
            Some(piece) => if piece.piece_type != PieceType::King {
                return false;
            } else {
                piece
            },
            None => return false, // No piece has been selected to be moved
        };
        let end_opt = board[end_pos.0][end_pos.1];

        if piece.castle.unwrap() {
            if move_vec.1 == 0 {
                if move_vec.0 == 2 {

                }
            }
        }

        if !(move_vec.0.abs() == 1 || move_vec.1.abs() == -1) || (move_vec.0 == 0 && move_vec.1 == 0) {
            return false;
        }

        // HAVE TO CHECK IF MOVING KING RESULT IN MATE !!
        

        match piece.color {
            Color::White => match end_opt {
                Some(end_piece) => match end_piece.color {
                    Color::White => return false,
                    Color::Black => return true,
                },
                None => return true,
            },
            Color::Black => match end_opt {
                Some(end_piece) => match end_piece.color {
                    Color::White => return true,
                    Color::Black => return false,
                },
                None => return true,
            },
        }
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

    match board[0][1] {
        Some(piece) => println!("Result : {}", (piece.check_move)(&board, (0, 1), (0, 2))),
        None => println!("Error, no piece selected"),
    }

    display_board(&board);

}

pub fn mut_pieces(board: &mut [[Option<Piece>; 8]; 8], pos1: (usize, usize), pos2: (usize, usize)) -> (&mut Option<Piece>, &mut Option<Piece>) {
    
    let pieces: (&mut Option<Piece>, &mut Option<Piece>);
    
    let mid = if pos1.0 < pos2.0 {
        pos1.0 + 1
    } else if pos2.0 < pos1.0 {
        pos2.0 + 1
    } else {
        0
    };

    if mid == 0 {
        let mid2 = if pos1.1 < pos2.1 {
            pos1.1 + 1
        } else if pos2.1 < pos1.1 {
            pos2.1 + 1
        } else {
            panic!(); // We are trying to get mutable references to the same memory location
        };
        let (first_half, last_half) = board[pos1.0].split_at_mut(mid2);
        pieces = (&mut first_half[pos1.1], &mut last_half[pos2.1]);
    } else {
        let (first_half, last_half) = board.split_at_mut(mid);
        pieces = (&mut first_half[pos1.0][pos1.1], &mut last_half[pos2.0][pos2.1]);
    }

    pieces

}

pub fn display_board(board: &[[Option<Piece>; 8]; 8]) {

    let mut display: String = "".to_string();

    for i in (0..8).rev() {
        display.push_str(&format!("{} ", i + 1));
        for j in 0..8 {
            match board[j][i] {
                Some(piece) => display.push(match (piece.piece_type, piece.color) {
                    (PieceType::Pawn, Color::White) => '♙',
                    (PieceType::Pawn, Color::Black) => '♟',
                    (PieceType::Rook, Color::White) => '♖',
                    (PieceType::Rook, Color::Black) => '♜',
                    (PieceType::Knight, Color::White) => '♘',
                    (PieceType::Knight, Color::Black) => '♞',
                    (PieceType::Bishop, Color::White) => '♗',
                    (PieceType::Bishop, Color::Black) => '♝',
                    (PieceType::Queen, Color::White) => '♕',
                    (PieceType::Queen, Color::Black) => '♛',
                    (PieceType::King, Color::White) => '♔',
                    (PieceType::King, Color::Black) => '♚',
                }),
                None => display.push(' '),
            }
            display.push(' ');
        }
        display.push('\n');
    }
    display.push_str("  A B C D E F G H");

    println!("{}", display);

}