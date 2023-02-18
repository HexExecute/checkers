use std::fmt;

pub enum Side {
    Left,
    Right
}

pub enum Direction {
    Forward(Side),
    Backward(Side)
}

#[derive(Clone, Copy)]
pub enum Piece {
    Red,
    RedKing,
    Black,
    BlackKing,
    Empty
}

struct Board {
    data: Vec<Piece>
}

impl Board {
    fn new() -> Self {
        Self {
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

    fn get(&self, x: u8, y: u8) -> Piece {
        self.data[(y * 8 + x) as usize]
    }

    fn advance(&mut self, (x, y): (u8, u8), direction: Direction) {
        let piece = self.get(x, y);

        match piece {
            Piece::Red => match direction {
                Direction::Forward(Side::Left) => {},
                Direction::Forward(Side::Right) => {},
                _ => return
            },
            Piece::RedKing => match direction {
                Direction::Forward(Side::Left) => {},
                Direction::Forward(Side::Right) => {},
                Direction::Backward(Side::Left) => {},
                Direction::Backward(Side::Right) => {}
            },
            Piece::Black => match direction {
                Direction::Forward(Side::Left) => {},
                Direction::Forward(Side::Right) => {},
                _ => return
            },
            Piece::BlackKing => match direction {
                Direction::Forward(Side::Left) => {},
                Direction::Forward(Side::Right) => {},
                Direction::Backward(Side::Left) => {},
                Direction::Backward(Side::Right) => {}
            },
            Piece::Empty => return
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(match self {
            Piece::Red => "R",
            Piece::RedKing => "RK",
            Piece::Black => "B",
            Piece::BlackKing => "BK",
            Piece::Empty => "E"
        })
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, v) in self.data.iter().enumerate() {
            if i % 8 == 0 { f.write_str("\n\n")?; }
            write!(f, "{v:<3}")?;
        }
        
        Ok(())
    }
}

#[test]
fn test() {
    let board = Board::new();
    println!("{board}");
}
