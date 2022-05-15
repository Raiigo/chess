use Piece::*;
use Color::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, Copy, Debug)]
pub enum Piece {
    Pawn {
        color: Color,
        jump: bool,
        just_jumped: bool,
    },
    Rook {
        color: Color,
        castle: bool,
    },
    Knight {
        color: Color,
    },
    Bishop {
        color: Color,
    },
    Queen {
        color: Color,
    },
    King {
        color: Color,
        castle: bool,
    },
}

impl Piece {
    pub fn get_color(&self) -> Color {
        match self {
            Pawn { color, jump, just_jumped } => return *color,
            Rook { color, castle } => return *color,
            Knight { color } => return *color,
            Bishop { color } => return *color,
            Queen { color } => return *color,
            King { color, castle } => return *color,
        }
    }
}

pub fn init_board() -> [[Option<Piece>; 8]; 8] {
    return [[Some(Rook { color: White, castle: false }), Some(Pawn { color: White, jump: false, just_jumped: false }), None, None, None, None, Some(Pawn { color: Black, jump: false, just_jumped: false }), Some(Rook { color: Black, castle: false })],
            [Some(Knight { color: White })             , Some(Pawn { color: White, jump: false, just_jumped: false }), None, None, None, None, Some(Pawn { color: Black, jump: false, just_jumped: false }), Some(Knight { color: Black })], 
            [Some(Bishop { color: White })             , Some(Pawn { color: White, jump: false, just_jumped: false }), None, None, None, None, Some(Pawn { color: Black, jump: false, just_jumped: false }), Some(Bishop { color: Black })], 
            [Some(Queen { color: White })              , Some(Pawn { color: White, jump: false, just_jumped: false }), None, None, None, None, Some(Pawn { color: Black, jump: false, just_jumped: false }), Some(Queen { color: Black })], 
            [Some(King { color: White, castle: false }), Some(Pawn { color: White, jump: false, just_jumped: false }), None, None, None, None, Some(Pawn { color: Black, jump: false, just_jumped: false }), Some(King { color: Black, castle: false })], 
            [Some(Bishop { color: White })             , Some(Pawn { color: White, jump: false, just_jumped: false }), None, None, None, None, Some(Pawn { color: Black, jump: false, just_jumped: false }), Some(Bishop { color: Black })], 
            [Some(Knight { color: White })             , Some(Pawn { color: White, jump: false, just_jumped: false }), None, None, None, None, Some(Pawn { color: Black, jump: false, just_jumped: false }), Some(Knight { color: Black })], 
            [Some(Rook { color: White, castle: false }), Some(Pawn { color: White, jump: false, just_jumped: false }), None, None, None, None, Some(Pawn { color: Black, jump: false, just_jumped: false }), Some(Rook { color: Black, castle: false })]]
}

pub fn move_info(board: [[Option<Piece>; 8]; 8], current_pos: (usize, usize), target_pos: (usize, usize)) -> (bool, Option<Piece>) {
    if target_pos.0 > 7 || target_pos.1 > 7 {
        return (false, None)
    }

    let relative_move = (target_pos.0 as i8 - current_pos.0 as i8, target_pos.1 as i8 - current_pos.1 as i8);
    println!("{:?}", relative_move);

    if relative_move.0 == 0 && relative_move.1 == 0 {
        return (false, None)
    }

    let target_piece: Option<Piece> = board[target_pos.0][target_pos.1];
    println!("{:?}", target_piece);

    match board[current_pos.0][current_pos.1] {
        Some(piece) => {
            match piece {
                Pawn { color, jump, just_jumped } => {
                    match color {
                        White => { // White Pawn
                            if relative_move == (0, 1) {
                                if board[current_pos.0][current_pos.1 + 1].is_none() {
                                    return (true, None)
                                } else {
                                    return (false, None)
                                }
                            } else if relative_move == (0, 2) {
                                if jump == false && board[current_pos.0][current_pos.1 + 1].is_none() && board[current_pos.0][current_pos.1 + 2].is_none() {
                                    return (true, None)
                                } else {
                                    return (false, None)
                                }
                            } else if relative_move == (1, 1) {
                                match target_piece {
                                    Some(t_piece) => {
                                        if t_piece.get_color() == Black {
                                            return (true, Some(t_piece))
                                        } else {
                                            return (false, None)
                                        }
                                    },
                                    None => {
                                        return (false, None)
                                    }
                                }
                            } else if relative_move == (-1, 1) {
                                match target_piece {
                                    Some(t_piece) => {
                                        if t_piece.get_color() == Black {
                                            return (true, Some(t_piece))
                                        } else {
                                            return (false, None)
                                        }
                                    },
                                    None => {
                                        return (false, None)
                                    }
                                }
                            } else {
                                return (false, None)
                            }
                        },
                        Black => { // Black Pawn
                            if relative_move == (0, -1) {
                                if board[current_pos.0][current_pos.1 - 1].is_none() {
                                    return (true, None)
                                } else {
                                    return (false, None)
                                }
                            } else if relative_move == (0, -2) {
                                if jump == false && board[current_pos.0][current_pos.1 - 1].is_none() && board[current_pos.0][current_pos.1 - 2].is_none() {
                                    return (true, None)
                                } else {
                                    return (false, None)
                                }
                            } else if relative_move == (1, -1) {
                                match target_piece {
                                    Some(t_piece) => {
                                        if t_piece.get_color() == White {
                                            return (true, Some(t_piece))
                                        } else {
                                            return (false, None)
                                        }
                                    },
                                    None => {
                                        return (false, None)
                                    }
                                }
                            } else if relative_move == (-1, -1) {
                                match target_piece {
                                    Some(t_piece) => {
                                        if t_piece.get_color() == White {
                                            return (true, Some(t_piece))
                                        } else {
                                            return (false, None)
                                        }
                                    },
                                    None => {
                                        return (false, None)
                                    }
                                }
                            } else {
                                return (false, None)
                            }
                        }
                    }
                },
                Rook { color, castle } => {
                    if relative_move.0 != 0 && relative_move.1 != 0 {
                        return (false, None)
                    }
                    if relative_move.0 > 0 {
                        for i in 1..relative_move.0 {
                            println!("Checking pos ({:?}, {:?}) : {:?}", current_pos.0 + i as usize, current_pos.1, board[current_pos.0 + i as usize][current_pos.1]);
                            if !board[current_pos.0 + i as usize][current_pos.1].is_none() {
                                return (false, None)
                            }
                        }
                    } else if relative_move.0 < 0 {
                        for i in 1..relative_move.0 {
                            println!("Checking pos ({:?}, {:?}) : {:?}", current_pos.0 - i as usize, current_pos.1, board[current_pos.0 - i as usize][current_pos.1]);
                            if !board[current_pos.0 - i as usize][current_pos.1].is_none() {
                                return (false, None)
                            }
                        }
                    } else if relative_move.1 > 0 {
                        for i in 1..relative_move.1 {
                            println!("Checking pos ({:?}, {:?}) : {:?}", current_pos.0, current_pos.1 + i as usize, board[current_pos.0][current_pos.1 + i as usize]);
                            if !board[current_pos.0][current_pos.1 + i as usize].is_none() {
                                return (false, None)
                            }
                        }
                    } else if relative_move.1 < 0 {
                        for i in 1..relative_move.1 {
                            println!("Checking pos ({:?}, {:?}) : {:?}", current_pos.0, current_pos.1 - i as usize, board[current_pos.0][current_pos.1 - i as usize]);
                            if !board[current_pos.0][current_pos.1 - i as usize].is_none() {
                                return (false, None)
                            }
                        }
                    } else {
                        panic!()
                    }
                    match color {
                        White => {
                            match target_piece {
                                Some(t_piece) => {
                                    if t_piece.get_color() == Black {
                                        return (true, Some(t_piece))
                                    } else {
                                        return (false, None)
                                    }
                                },
                                None => {
                                    return (true, None)
                                }
                            }
                        },
                        Black => {
                            match target_piece {
                                Some(t_piece) => {
                                    if t_piece.get_color() == White {
                                        return (true, Some(t_piece))
                                    } else {
                                        return (false, None)
                                    }
                                },
                                None => {
                                    return (true, None)
                                }
                            }
                        },
                    }
                },
                Knight { color } => {
                    todo!()
                },
                Bishop { color } => {
                    todo!()
                },
                Queen { color } => {
                    todo!()
                },
                King { color, castle } => {
                    todo!()
                },
                
            }
        },
        None => return (false, None)
    }
}

fn main() {

    let mut board = init_board();
    println!("{:?}", board[0][0]);
    println!("{:?}", move_info(board, (0, 0), (0, 4)));

}
