use std::fmt;

#[derive(Clone, Copy)]
pub enum Piece {
    Red,
    Black,
    Empty
}

struct Board {
    turn: bool,
    data: Vec<Piece>
}

impl Board {
    fn new() -> Self {
        Self {
            turn: false,
            data: {
                let mut vec = vec![Piece::Empty; 64];

                for row in 0..8 {
                    for col in 0..8 {
                        let index = row * 8 + col;

                        if (row + col) % 2 == 0 {
                            continue; // skip even squares
                        }

                        if row < 3 {
                            vec[index] = Piece::Red;
                        } else if row > 4 {
                            vec[index] = Piece::Black;
                        }
                    }
                }
                vec
            }
        }
    }

    fn advance(&mut self) {
        match self.turn {
            true => {},
            false => {}
        }

        self.turn = !self.turn;
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string = "".to_string();

        for (i, v) in self.data.iter().enumerate() {
            if i % 8 == 0 { string += "\n"; }
            string += &(match v {
                Piece::Red => "R",
                Piece::Black => "B",
                Piece::Empty => "E"
            }.to_owned() + " ")[..];
        }
        
        write!(f, "{string}")
    }
}

#[test]
fn test() {
    let board = Board::new();
    println!("{board}");
}
