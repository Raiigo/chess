use std::{io::{stdin, stdout, Write}, fmt::Debug};

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
    exec_move: fn(&mut [[Option<Piece>; 8]; 8], (usize, usize), (usize, usize)),
}

impl Piece {
    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl Debug for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Piece").field("color", &self.color).field("piece_type", &self.piece_type).field("castle", &self.castle).field("jump", &self.jump).field("en_passant", &self.en_passant).finish()
    }
}

const PAWN: Piece = Piece {
    color: Color::White,
    piece_type: PieceType::Pawn,
    castle: None,
    jump: Some(true),
    en_passant: Some(false),
    check_move: |board: &[[Option<Piece>; 8]; 8], start_pos: (usize, usize), end_pos: (usize, usize)| -> bool {

        let is_legit;

        let move_vec: (isize, isize) = ((end_pos.0 as isize - start_pos.0 as isize), (end_pos.1 as isize - start_pos.1 as isize));
        let piece = match board[start_pos.0][start_pos.1] {
            Some(piece) => if piece.piece_type != PieceType::Pawn {
                return false;
            } else {
                piece
            },
            None => return false, // No piece has been selected to be moved
        };
        let current_color = piece.color;
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
                                is_legit = true;
                            } else {
                                return false;
                            },
                            None => {
                                let en_passant_piece_opt = board[end_pos.0][end_pos.1 - 1];
                                match en_passant_piece_opt {
                                    Some(en_passant_piece) => {
                                        if en_passant_piece.piece_type == PieceType::Pawn {
                                            if en_passant_piece.en_passant.unwrap() && en_passant_piece.color == Color::Black {
                                                is_legit = true;
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
                            None => is_legit = true,
                        }
                    }
                } else if move_vec.1 == 2 && piece.jump.unwrap() {
                    match end_opt {
                        Some(_) => return false,
                        None => {
                            match board[end_pos.0][end_pos.1 - 1] {
                                Some(_) => return false,
                                None => is_legit = true,
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
                                is_legit = true;
                            } else {
                                return false;
                            },
                            None => {
                                let en_passant_piece_opt = board[end_pos.0][end_pos.1 + 1];
                                match en_passant_piece_opt {
                                    Some(en_passant_piece) => {
                                        if en_passant_piece.piece_type == PieceType::Pawn {
                                            if en_passant_piece.en_passant.unwrap() && en_passant_piece.color == Color::White {
                                                is_legit = true;
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
                            None => is_legit = true,
                        }
                    }
                } else if move_vec.1 == -2 && piece.jump.unwrap() {
                    match end_opt {
                        Some(_) => return false,
                        None => {
                            match board[end_pos.0][end_pos.1 + 1] {
                                Some(_) => return false,
                                None => is_legit = true,
                            }
                        },
                    }
                } else {
                    return false;
                }
            },
        }

        if is_legit {
            let mut king_pos = (0, 0);
            for i in 0..8 {
                for j in 0..8 {
                    match board[i][j] {
                        Some(piece) => if piece.piece_type == PieceType::King && piece.color == current_color {
                            king_pos = (i, j);
                        } else {
                            continue;
                        },
                        None => continue,
                    }
                }
            }
            dbg!(check_check(board, start_pos, end_pos, king_pos));
            if !check_check(board, start_pos, end_pos, king_pos) {
                return true;
            } else {
                return false;
            }
        }

        return false;
    },
    exec_move: |board: &mut [[Option<Piece>; 8]; 8], start_pos: (usize, usize), end_pos: (usize, usize)| {
        if !(PAWN.check_move)(board, start_pos, end_pos) {
            return;
        }
        let move_vec: (isize, isize) = ((end_pos.0 as isize - start_pos.0 as isize), (end_pos.1 as isize - start_pos.1 as isize));
        if move_vec.1.abs() == 2 {
            board[end_pos.0][end_pos.1] = board[start_pos.0][start_pos.1];
            board[start_pos.0][start_pos.1] = None;
            board[end_pos.0][end_pos.1].unwrap().jump = Some(false);
            board[end_pos.0][end_pos.1].unwrap().en_passant = Some(true);
        } else {
            board[end_pos.0][end_pos.1] = board[start_pos.0][start_pos.1];
            board[start_pos.0][start_pos.1] = None;
        }
    },
};
const ROOK: Piece = Piece {
    color: Color::White,
    piece_type: PieceType::Rook,
    castle: Some(true),
    jump: None,
    en_passant: None,
    check_move: |board: &[[Option<Piece>; 8]; 8], start_pos: (usize, usize), end_pos: (usize, usize)| -> bool {

        let is_legit;

        let move_vec: (isize, isize) = ((end_pos.0 as isize - start_pos.0 as isize), (end_pos.1 as isize - start_pos.1 as isize));
        let piece = match board[start_pos.0][start_pos.1] {
            Some(piece) => piece,
            None => return false, // No piece has been selected to be moved
        };
        let current_color = piece.color;
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
                        Color::Black => is_legit = true,
                    },
                    None => is_legit = true,
                }
            },
            Color::Black => {
                match end_opt {
                    Some(end_piece) => match end_piece.color {
                        Color::White => is_legit = true,
                        Color::Black => return false,
                    },
                    None => is_legit = true,
                }
            },
        }

        if is_legit {
            let mut king_pos = (0, 0);
            for i in 0..8 {
                for j in 0..8 {
                    match board[i][j] {
                        Some(piece) => if piece.piece_type == PieceType::King && piece.color == current_color {
                            king_pos = (i, j);
                        } else {
                            continue;
                        },
                        None => continue,
                    }
                }
            }
            if !check_check(board, start_pos, end_pos, king_pos) {
                return true;
            } else {
                return false;
            }
        }

        return false;
    },
    exec_move: |board: &mut [[Option<Piece>; 8]; 8], start_pos: (usize, usize), end_pos: (usize, usize)| {
        if !(ROOK.check_move)(board, start_pos, end_pos) {
            return;
        }
        board[end_pos.0][end_pos.1] = board[start_pos.0][start_pos.1];
        board[start_pos.0][start_pos.1] = None;
        board[end_pos.0][end_pos.1].unwrap().castle = Some(false);
    },
};
const KNIGHT: Piece = Piece {
    color: Color::White,
    piece_type: PieceType::Knight,
    castle: None,
    jump: None,
    en_passant: None,
    check_move: |board: &[[Option<Piece>; 8]; 8], start_pos: (usize, usize), end_pos: (usize, usize)| -> bool {

        let is_legit;

        let move_vec: (isize, isize) = ((end_pos.0 as isize - start_pos.0 as isize), (end_pos.1 as isize - start_pos.1 as isize));
        let piece = match board[start_pos.0][start_pos.1] {
            Some(piece) => if piece.piece_type != PieceType::Knight {
                return false;
            } else {
                piece
            },
            None => return false, // No piece has been selected to be moved
        };
        let current_color = piece.color;
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
                        Color::Black => is_legit = true,
                    },
                    None => is_legit = true,
                }
            },
            Color::Black => {
                match end_opt {
                    Some(end_piece) => match end_piece.color {
                        Color::White => is_legit = true,
                        Color::Black => return false,
                    },
                    None => is_legit = true,
                }
            },
        }

        if is_legit {
            let mut king_pos = (0, 0);
            for i in 0..8 {
                for j in 0..8 {
                    match board[i][j] {
                        Some(piece) => if piece.piece_type == PieceType::King && piece.color == current_color {
                            king_pos = (i, j);
                        } else {
                            continue;
                        },
                        None => continue,
                    }
                }
            }
            if !check_check(board, start_pos, end_pos, king_pos) {
                return true;
            } else {
                return false;
            }
        }

        return false;
    },
    exec_move: |board: &mut [[Option<Piece>; 8]; 8], start_pos: (usize, usize), end_pos: (usize, usize)| {
        if !(KNIGHT.check_move)(board, start_pos, end_pos) {
            return;
        }
        board[end_pos.0][end_pos.1] = board[start_pos.0][start_pos.1];
        board[start_pos.0][start_pos.1] = None;
    },
};
const BISHOP: Piece = Piece {
    color: Color::White,
    piece_type: PieceType::Bishop,
    castle: None,
    jump: None,
    en_passant: None,
    check_move: |board: &[[Option<Piece>; 8]; 8], start_pos: (usize, usize), end_pos: (usize, usize)| -> bool {

        let is_legit;

        let move_vec: (isize, isize) = ((end_pos.0 as isize - start_pos.0 as isize), (end_pos.1 as isize - start_pos.1 as isize));
        let piece = match board[start_pos.0][start_pos.1] {
            Some(piece) => piece,
            None => return false, // No piece has been selected to be moved
        };
        let current_color = piece.color;
        let end_opt = board[end_pos.0][end_pos.1];

        if move_vec.0.abs() != move_vec.1.abs() || move_vec.0 == 0 {
            println!("Error 0");
            return false;
        }

        let mut pos_set: Vec<Option<Piece>> = vec![];

        let mut start_pos_mut = start_pos;

        if move_vec.0 < 0 {
            if move_vec.1 < 0 {
                while start_pos_mut.0 != end_pos.0 && start_pos_mut.1 != end_pos.1 {
                    start_pos_mut.0 -= 1;
                    start_pos_mut.1 -= 1;
                    pos_set.push(board[start_pos_mut.0][start_pos_mut.1]);
                }
            } else if move_vec.1 > 0 {
                while start_pos_mut.0 != end_pos.0 && start_pos_mut.1 != end_pos.1 {
                    start_pos_mut.0 -= 1;
                    start_pos_mut.1 += 1;
                    pos_set.push(board[start_pos_mut.0][start_pos_mut.1]);
                }
            }
        } else if move_vec.0 > 0 {
            if move_vec.1 < 0 {
                while start_pos_mut.0 != end_pos.0 && start_pos_mut.1 != end_pos.1 {
                    start_pos_mut.0 += 1;
                    start_pos_mut.1 -= 1;
                    pos_set.push(board[start_pos_mut.0][start_pos_mut.1]);
                }
            } else if move_vec.1 > 0 {
                while start_pos_mut.0 != end_pos.0 && start_pos_mut.1 != end_pos.1 {
                    start_pos_mut.0 += 1;
                    start_pos_mut.1 += 1;
                    pos_set.push(board[start_pos_mut.0][start_pos_mut.1]);
                }
            }
        }

        for i in 0..(pos_set.len() - 1) {
            match pos_set[i] {
                Some(_) => {
                    println!("Error 1");
                    return false;
                },
                None => continue,
            }
        }

        match piece.color {
            Color::White => {
                match end_opt {
                    Some(end_piece) => match end_piece.color {
                        Color::White => {
                            println!("Error 2");
                            return false;
                        },
                        Color::Black => is_legit = true,
                    },
                    None => is_legit = true,
                }
            },
            Color::Black => {
                match end_opt {
                    Some(end_piece) => match end_piece.color {
                        Color::White => is_legit = true,
                        Color::Black => {
                            println!("Error 3");
                            return false;
                        },
                    },
                    None => is_legit = true,
                }
            },
        }

        if is_legit {
            let mut king_pos = (0, 0);
            for i in 0..8 {
                for j in 0..8 {
                    match board[i][j] {
                        Some(piece) => if piece.piece_type == PieceType::King && piece.color == current_color {
                            king_pos = (i, j);
                        } else {
                            continue;
                        },
                        None => continue,
                    }
                }
            }
            if !check_check(board, start_pos, end_pos, king_pos) {
                return true;
            } else {
                return false;
            }
        }

        return false;

    },
    exec_move: |board: &mut [[Option<Piece>; 8]; 8], start_pos: (usize, usize), end_pos: (usize, usize)| {
        if !(BISHOP.check_move)(board, start_pos, end_pos) {
            return;
        }
        board[end_pos.0][end_pos.1] = board[start_pos.0][start_pos.1];
        board[start_pos.0][start_pos.1] = None;
    },
};
const QUEEN: Piece = Piece {
    color: Color::White,
    piece_type: PieceType::Queen,
    castle: None,
    jump: None,
    en_passant: None,
    check_move: |board: &[[Option<Piece>; 8]; 8], start_pos: (usize, usize), end_pos: (usize, usize)| -> bool {
        if (ROOK.check_move)(board, start_pos, end_pos) || (BISHOP.check_move)(board, start_pos, end_pos) {
            return true;
        } else {
            return false;
        }
    },
    exec_move: |board: &mut [[Option<Piece>; 8]; 8], start_pos: (usize, usize), end_pos: (usize, usize)| {
        if !(QUEEN.check_move)(board, start_pos, end_pos) {
            return;
        }
        board[end_pos.0][end_pos.1] = board[start_pos.0][start_pos.1];
        board[start_pos.0][start_pos.1] = None;
    },
};
const KING: Piece = Piece {
    color: Color::White,
    piece_type: PieceType::King,
    castle: Some(true),
    jump: None,
    en_passant: None,
    check_move: |board: &[[Option<Piece>; 8]; 8], start_pos: (usize, usize), end_pos: (usize, usize)| -> bool {

        let is_legit;

        let move_vec: (isize, isize) = ((end_pos.0 as isize - start_pos.0 as isize), (end_pos.1 as isize - start_pos.1 as isize));
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

        if !(move_vec.0.abs() == 1 || move_vec.1.abs() == 1) || (move_vec.0 == 0 && move_vec.1 == 0) {
            return false;
        }

        // HAVE TO CHECK IF MOVING KING RESULT IN MATE !!
        

        match piece.color {
            Color::White => match end_opt {
                Some(end_piece) => match end_piece.color {
                    Color::White => return false,
                    Color::Black => is_legit = true,
                },
                None => is_legit = true,
            },
            Color::Black => match end_opt {
                Some(end_piece) => match end_piece.color {
                    Color::White => is_legit = true,
                    Color::Black => return false,
                },
                None => is_legit = true,
            },
        }

        if is_legit {
            if !check_check(board, start_pos, end_pos, end_pos) {
                return true;
            } else {
                return false;
            }
        }

        return false;

    },
    exec_move: |board: &mut [[Option<Piece>; 8]; 8], start_pos: (usize, usize), end_pos: (usize, usize)| {
        if !(KING.check_move)(board, start_pos, end_pos) {
            return;
        }
        board[end_pos.0][end_pos.1] = board[start_pos.0][start_pos.1];
        board[start_pos.0][start_pos.1] = None;
    },
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

    // match board[0][1] {
    //     Some(piece) => println!("Result : {}", (piece.check_move)(&board, (0, 1), (0, 2))),
    //     None => println!("Error, no piece selected"),
    // }

    loop {
        display_board(&board);
        println!();
        let mut move_expr = String::new();
        print!("-> ");
        match stdout().flush() {
            Ok(_) => (),
            Err(_) => panic!(),
        };
        stdin().read_line(&mut move_expr).expect("Error while reading stdin");
        if let Some('\n') = move_expr.chars().next_back() {
            move_expr.pop();
        }
        if let Some('\r') = move_expr.chars().next_back() {
            move_expr.pop();
        }
        let (start_pos, end_pos) = match parse_move(&move_expr) {
            Some(t) => t,
            None => continue,
        };
        match board[start_pos.0][start_pos.1] {
            Some(piece) => match piece.piece_type {
                PieceType::Pawn => {
                    if (PAWN.check_move)(&board, start_pos, end_pos) {
                        (PAWN.exec_move)(&mut board, start_pos, end_pos);
                    } else {
                        println!("Enter a valid move");
                        continue
                    }
                },
                PieceType::Rook => {
                    if (ROOK.check_move)(&board, start_pos, end_pos) {
                        (ROOK.exec_move)(&mut board, start_pos, end_pos);
                    } else {
                        println!("Enter a valid move");
                        continue
                    }
                },
                PieceType::Knight => {
                    if (KNIGHT.check_move)(&board, start_pos, end_pos) {
                        (KNIGHT.exec_move)(&mut board, start_pos, end_pos);
                    } else {
                        println!("Enter a valid move");
                        continue
                    }
                },
                PieceType::Bishop => {
                    if (BISHOP.check_move)(&board, start_pos, end_pos) {
                        (BISHOP.exec_move)(&mut board, start_pos, end_pos);
                    } else {
                        println!("Enter a valid move");
                        continue
                    }
                },
                PieceType::Queen => {
                    if (QUEEN.check_move)(&board, start_pos, end_pos) {
                        (QUEEN.exec_move)(&mut board, start_pos, end_pos);
                    } else {
                        println!("Enter a valid move");
                        continue
                    }
                },
                PieceType::King => {
                    if (KING.check_move)(&board, start_pos, end_pos) {
                        (KING.exec_move)(&mut board, start_pos, end_pos);
                    } else {
                        println!("Enter a valid move");
                        continue
                    }
                },
            },
            None => {
                println!("You have to move a piece");
                continue
            },
        }
    }

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

pub fn parse_move(expr: &str) -> Option<((usize, usize), (usize, usize))> {
    let pos_str: Vec<&str> = expr.split(' ').collect();
    if pos_str.len() != 2 {
        return None;
    } else {
        let start_pos_str: Vec<char> = pos_str[0].chars().collect();
        let start_pos_1 = match start_pos_str[0] {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            'D' => 3,
            'E' => 4,
            'F' => 5,
            'G' => 6,
            'H' => 7,
            _ => return None,
        } as usize;
        let start_pos_2 = match start_pos_str[1] {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            _ => return None,
        } as usize;
        let end_pos_str: Vec<char> = pos_str[1].chars().collect();
        let end_pos_1 = match end_pos_str[0] {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            'D' => 3,
            'E' => 4,
            'F' => 5,
            'G' => 6,
            'H' => 7,
            _ => return None,
        } as usize;
        let end_pos_2 = match end_pos_str[1] {
            '1' => 0,
            '2' => 1,
            '3' => 2,
            '4' => 3,
            '5' => 4,
            '6' => 5,
            '7' => 6,
            '8' => 7,
            _ => return None,
        } as usize;
        return Some(((start_pos_1, start_pos_2), (end_pos_1, end_pos_2)));
    }
}

// Return true if there is check
pub fn check_check(board: &[[Option<Piece>; 8]; 8], start_pos: (usize, usize), end_pos: (usize, usize), king_pos: (usize, usize)) -> bool { // What a name ...
    let mut temp_board = board.clone();
    temp_board[end_pos.0][end_pos.1] = temp_board[start_pos.0][start_pos.1];
    temp_board[start_pos.0][start_pos.1] = None;

    println!("TEMP BOARD :");
    display_board(&temp_board);

    let king_color = temp_board[king_pos.0][king_pos.1].unwrap().color;
    let enemy_color = match king_color {
        Color::White => Color::Black,
        Color::Black => Color::White,
    };

    let mut knight_pos: Vec<(isize, isize)> = vec![];
    knight_pos.push((king_pos.0 as isize + 2, king_pos.0 as isize + 1));
    knight_pos.push((king_pos.0 as isize + 2, king_pos.0 as isize - 1));
    knight_pos.push((king_pos.0 as isize - 2, king_pos.0 as isize + 1));
    knight_pos.push((king_pos.0 as isize - 2, king_pos.0 as isize - 1));
    knight_pos.push((king_pos.0 as isize + 1, king_pos.0 as isize + 2));
    knight_pos.push((king_pos.0 as isize - 1, king_pos.0 as isize + 2));
    knight_pos.push((king_pos.0 as isize + 1, king_pos.0 as isize - 2));
    knight_pos.push((king_pos.0 as isize - 1, king_pos.0 as isize - 2));
    knight_pos = knight_pos.into_iter().filter(|pos| {
        if pos.0 < 0 || pos.0 > 7 || pos.1 < 0 || pos.1 > 7 {
            false
        } else {
            true
        }
    }).collect();
    for pos in knight_pos {
        match temp_board[pos.0 as usize][pos.1 as usize]  {
            Some(piece) => if piece.piece_type == PieceType::Knight && piece.color == enemy_color {
                return true;
            } else {
                continue;
            },
            None => continue,
        }
    }
    match king_color {
        Color::White => {
            let mut pawn_pos = vec![(king_pos.0 as isize + 1, king_pos.1 as isize + 1), (king_pos.0 as isize - 1, king_pos.1 as isize + 1)];
            
            pawn_pos = pawn_pos.into_iter().filter(|pos| {
                if pos.0 < 0 || pos.0 > 7 || pos.1 < 0 || pos.1 > 7 {
                    false
                } else {
                    true
                }
            }).collect();

            for pos in pawn_pos {
                let pos = (pos.0 as usize, pos.1 as usize);
                match temp_board[pos.0][pos.1] {
                    Some(piece) => if piece.piece_type == PieceType::Pawn && piece.color == enemy_color {
                        return true;
                    },
                    None => continue,
                }
            }
        },
        Color::Black => {
            let mut pawn_pos = vec![(king_pos.0 as isize + 1, king_pos.1 as isize - 1), (king_pos.0 as isize - 1, king_pos.1 as isize - 1)];

            pawn_pos = pawn_pos.into_iter().filter(|pos| {
                if pos.0 < 0 || pos.0 > 7 || pos.1 < 0 || pos.1 > 7 {
                    false
                } else {
                    true
                }
            }).collect();

            for pos in pawn_pos {
                let pos = (pos.0 as usize, pos.1 as usize);
                match temp_board[pos.0][pos.1] {
                    Some(piece) => if piece.piece_type == PieceType::Pawn && piece.color == enemy_color {
                        return true;
                    },
                    None => continue,
                }
            }
        },
    }

    // let rook_pos = vec![(king_pos.0 + 1)..7, 0..king_pos.0, 0..king_pos.1, king_pos.1..7];
    if !(((king_pos.0 + 1)..7).is_empty()) {
        for i in (king_pos.0 + 1)..7 {
            match temp_board[i][king_pos.1] {
                Some(piece) => if (piece.piece_type == PieceType::Rook || piece.piece_type == PieceType::Queen) && piece.color == enemy_color {
                    return true;
                } else {
                    break;
                },
                None => continue,
            }
        }
    }
    if !((0..king_pos.0).is_empty()) {
        for i in (0..king_pos.0).rev() {
            match temp_board[i][king_pos.1] {
                Some(piece) => if (piece.piece_type == PieceType::Rook || piece.piece_type == PieceType::Queen) && piece.color == enemy_color {
                    return true;
                } else {
                    break;
                },
                None => continue,
            }
        }
    }
    if !((0..king_pos.1).is_empty()) {
        for i in (0..king_pos.1).rev() {
            match temp_board[king_pos.0][i] {
                Some(piece) => if (piece.piece_type == PieceType::Rook || piece.piece_type == PieceType::Queen) && piece.color == enemy_color {
                    return true;
                } else {
                    break;
                },
                None => continue,
            }
        }
    }
    if !(((king_pos.1 + 1)..7).is_empty()) {
        for i in (king_pos.1 + 1)..7 {
            match temp_board[king_pos.0][i] {
                Some(piece) => if (piece.piece_type == PieceType::Rook || piece.piece_type == PieceType::Queen) && piece.color == enemy_color {
                    return true;
                } else {
                    break;
                },
                None => continue,
            }
        }
    }

    let mut bishop_pos_pp: Vec<(usize, usize)> = vec![]; // positive, positive
    let mut current_pos = king_pos.clone();
    while current_pos.0 < 7 && current_pos.1 < 7 { // +, +
        current_pos.0 += 1;
        current_pos.1 += 1;
        bishop_pos_pp.push(current_pos);
    }
    current_pos = king_pos.clone();
    let mut bishop_pos_pn: Vec<(usize, usize)> = vec![]; // positive, negative
    while current_pos.0 < 7 && current_pos.1 > 0 { // +, -
        current_pos.0 += 1;
        current_pos.1 -= 1;
        bishop_pos_pn.push(current_pos);
    }
    current_pos = king_pos.clone();
    let mut bishop_pos_np: Vec<(usize, usize)> = vec![]; // negative, positive
    while current_pos.0 > 0 && current_pos.1 < 7 { // -, +
        current_pos.0 -= 1;
        current_pos.1 += 1;
        bishop_pos_np.push(current_pos);
    }
    current_pos = king_pos.clone();
    let mut bishop_pos_nn: Vec<(usize, usize)> = vec![]; // negative, negative
    while current_pos.0 > 0 && current_pos.1 > 0 { // -, -
        current_pos.0 -= 1;
        current_pos.1 -= 1;
        bishop_pos_nn.push(current_pos);
    }
    dbg!(&bishop_pos_pp);
    dbg!(&bishop_pos_pn);
    dbg!(&bishop_pos_np);
    dbg!(&bishop_pos_nn);
    println!("from ({}, {})", king_pos.0, king_pos.1);
    for pos in bishop_pos_pp {
        println!("to ({}, {})", pos.0, pos.1);
        match temp_board[pos.0][pos.1] {
            Some(piece) => if (piece.piece_type == PieceType::Bishop || piece.piece_type == PieceType::Queen) && piece.color == enemy_color {
                return true;
            } else {
                break;
            },
            None => continue,
        }
    }
    for pos in bishop_pos_pn {
        println!("to ({}, {})", pos.0, pos.1);
        match temp_board[pos.0][pos.1] {
            Some(piece) => if (piece.piece_type == PieceType::Bishop || piece.piece_type == PieceType::Queen) && piece.color == enemy_color {
                return true;
            } else {
                break;
            },
            None => continue,
        }
    }
    for pos in bishop_pos_np {
        println!("to ({}, {})", pos.0, pos.1);
        match temp_board[pos.0][pos.1] {
            Some(piece) => if (piece.piece_type == PieceType::Bishop || piece.piece_type == PieceType::Queen) && piece.color == enemy_color {
                return true;
            } else {
                break;
            },
            None => continue,
        }
    }
    for pos in bishop_pos_nn {
        println!("to ({}, {})", pos.0, pos.1);
        match temp_board[pos.0][pos.1] {
            Some(piece) => if (piece.piece_type == PieceType::Bishop || piece.piece_type == PieceType::Queen) && piece.color == enemy_color {
                return true;
            } else {
                break;
            },
            None => continue,
        }
    }

    let mut enemy_king_pos = vec![(king_pos.0 as isize + 1, king_pos.1 as isize), (king_pos.0 as isize - 1, king_pos.1 as isize), (king_pos.0 as isize, king_pos.1 as isize + 1), (king_pos.0 as isize, king_pos.1 as isize - 1), (king_pos.0 as isize + 1, king_pos.1 as isize + 1), (king_pos.0 as isize - 1, king_pos.1 as isize + 1), (king_pos.0 as isize - 1, king_pos.1 as isize - 1), (king_pos.0 as isize + 1, king_pos.1 as isize - 1)];

    enemy_king_pos = enemy_king_pos.into_iter().filter(|pos| {
        if pos.0 < 0 || pos.0 > 7 || pos.1 < 0 || pos.1 > 7 {
            false
        } else {
            true
        }
    }).collect();

    for pos in enemy_king_pos {
        match temp_board[pos.0 as usize][pos.1 as usize] {
            Some(piece) => if piece.piece_type == PieceType::King && piece.color == enemy_color {
                return true;
            } else {
                continue;
            },
            None => continue,
        }
    }

    return false;
}