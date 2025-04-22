use std::io;

fn print_board(board: [[char; 3]; 3]) {
    println!("    a|b|c");
    for (i, row) in board.iter().enumerate() {
        println!("{i} = {}|{}|{}", row[0], row[1], row[2]);
    }
}


fn print_line(){
    println!("-------------------------------------");
}

fn validate_board(board: [[char; 3]; 3]) -> char {
    for i in 0..3 {
        // Search game in rows
        if board[i][0] == board[i][1] && board[i][1] == board[i][2] && board[i][0] != '-' {
            return board[i][0];
        }

        // Search game in columns
        if board[0][i] == board[1][i] && board[1][i] == board[2][i] && board[0][i] != '-' {
            return board[0][i];
        }
    }

    // Search game in diagonals
    if board[0][0] == board[1][1] && board[1][1] == board[2][2] && board[0][0] != '-' {
        return board[0][0];
    }
    if board[0][2] == board[1][1] && board[1][1] == board[2][0] && board[0][2] != '-' {
        return board[0][2];
    }

    for row in board {
        for item in row {
            if item == '-' {
                return '-';
            };
        }
    }

    // In case game doesn't have any moves, returns v for "velha"
    'v'
}

fn write_on_board(board: &mut [[char; 3]; 3], row: usize, column: usize, value: char) -> bool {
    if board[row][column] != '-' {
        println!("Space already taken, please select other");
        return false;
    }

    board[row][column] = value;
    true
}

fn main() {
    let mut board = [['-', '-', '-'], ['-', '-', '-'], ['-', '-', '-']];
    let mut validation :char;
    'game: loop {
        for letter in ['X', 'O'] {
            loop {
                print_line();
                println!("Time for {letter} to play!");
                print_board(board);
                println!("Select a position to play (eg. a0))");
                let mut guess = String::new();
                io::stdin().read_line(&mut guess).expect("Error reading line.");
                {
                    let column = &guess.chars().nth(0).unwrap();
                    let column: usize = match column {
                        'a' => 0,
                        'b' => 1,
                        'c' => 2,
                        _ => {
                            println!("Please pick a valid column");
                            continue;
                        }
                    };

                    let row = &guess.chars().nth(1).unwrap();
                    let row: usize = row.to_digit(10).unwrap() as usize;

                    if row > 2{
                        println!("Please insert a valid row.");
                        continue;
                    }

                    if write_on_board(&mut board, row, column, letter) {
                        break;
                    }
                }
            }

            validation = validate_board(board);
            if validation == 'v' || validation == 'X' || validation == 'O' {
                break 'game;
            }
        }
    }
    print_line();
    print_board(board);
    match validation {
        'v' => println!("It's a tie!"),
        'X' => println!("X is the big winner!"),
        'O' => println!("O is the big winner!"),
        _ => println!("Algo de errado não está certo!"),
    }
}
