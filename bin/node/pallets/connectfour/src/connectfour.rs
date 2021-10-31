// use rand::Rng;

pub struct Logic {
}

impl Logic {

    pub fn full(board: [[u8; 6]; 7]) -> bool {
        let y_pos = board[0].len() - 1;
        for x_pos in 0..board.len() {
            if board[x_pos][y_pos] == 0 {
                return false
            }
        }
        
        true
    }

    pub fn evaluate(board: [[u8; 6]; 7], player: u8) -> bool {
        // horizontalCheck 
        for y in 0..board[0].len() {
            for x in 0..board.len()-3 {
                if board[x][y] == player 
                && board[x+1][y] == player 
                && board[x+2][y] == player 
                && board[x+3][y] == player {
                    return true;
                }           
            }
        }
        
        // verticalCheck
        for y in 0..board[0].len()-3 {
            for x in 0..board.len() {
                if board[x][y] == player 
                && board[x][y+1] == player
                && board[x][y+2] == player
                && board[x][y+3] == player {
                    return true;
                }           
            }
        }
        
        // ascendingDiagonalCheck 
        for y in 0..board[0].len()-3 {
            for x in 3..board.len() {
                if board[x][y] == player 
                && board[x-1][y+1] == player
                && board[x-2][y+2] == player
                && board[x-3][y+3] == player {
                    return true;
                }
            }
        }

        // descendingDiagonalCheck 
        for y in 3..board[0].len() {
            for x in 3..board.len() {
                if board[x][y] == player 
                && board[x-1][y-1] == player
                && board[x-2][y-2] == player
                && board[x-3][y-3] == player {
                    return true;
                }
            }
        }
        return false;
    }
    
    pub fn add_stone(board: &mut [[u8; 6]; 7], column: u8, player: u8) -> bool {
        if board[column as usize][0] > 0 {
            return false;
        }
        let board_rows:usize = board[0].len();
        for y in 0..board_rows {
            let y_pos = board_rows - y - 1;
            if board[column as usize][y_pos] > 0 {
                continue;
            }
            board[column as usize][y_pos] = player;
            break;
        }
        true
    }
    
    // pub fn random_board() ->  [[u8; 6]; 7] {
    //     let mut board = [[0u8; 6]; 7];
    //     let mut rng = rand::thread_rng();
    //     let mut n1: u8 = rng.gen();
    //     n1 = n1 % 42;
    //     // add randomly stones to the board   
    //     for i in 0..n1 {
    //         let mut stone_set = true;
    //         loop {
    //             let mut column: u8 = rng.gen();
    //             column = column % 7;
    //             if Self::add_stone(&mut board, column, (i%2) + 1) {
    //                 break;
    //             }
    //         }
    //     }
    //     board
    // }
    
    // pub fn print_board(board: [[u8; 6]; 7]) {
    //     println!("   c0  c1  c2  c3  c4  c5  c6  ");
    //     println!("  + - + - + - + - + - + - + - +");
    //     for y in 0..board[0].len() {
    //         print!("r{:?}|", y);
    //         for x in 0..board.len() {
    //             match board[x][y] {
    //                 0 => print!("   |"),
    //                 1 => print!(" X |"),
    //                 2 => print!(" O |"),
    //                 _ => print!(" . |"),
    //             }
    //         }
    //         println!("");
    //         println!("  + - + - + - + - + - + - + - +");
    //     }
    // }

}

// fn main() {
    
//     loop {
//         let board = Logic::random_board();
//         if Logic::evaluate(board, 1) || Logic::evaluate(board, 2) {
//             println!("WE HAVE A WINNER !!!");
//             Logic::print_board(board);
//             break;
//         }
//     }
// }