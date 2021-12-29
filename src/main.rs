use ansi_term::Colour::RGB;
use indexmap::IndexSet;
use std::fmt;

#[derive(PartialEq, Eq, Hash)]
struct FixedPentomino {
    squares: [(i32, i32); 5],
}

impl FixedPentomino {
    fn transform(&self, n: i32) -> Self {
        let m = match n {
            0 => ((1, 0), (0, 1)),
            1 => ((0, -1), (1, 0)),
            2 => ((-1, 0), (0, -1)),
            3 => ((0, 1), (-1, 0)),
            4 => ((-1, 0), (0, 1)),
            5 => ((0, -1), (-1, 0)),
            6 => ((1, 0), (0, -1)),
            7 => ((0, 1), (1, 0)),
            _ => panic!(),
        };

        let mut s = self
            .squares
            .map(|(x, y)| (x * m.0 .0 + y * m.0 .1, x * m.1 .0 + y * m.1 .1));

        s.sort_by(|a, b| {
            if a.1 == b.1 {
                return a.0.cmp(&b.0);
            }
            a.1.cmp(&b.1)
        });

        // Transpose, such that (0, 0) is the first square on the first row. X-coordinates might
        // still be negative in further rows.
        let min_y = s.iter().map(|(_, y)| y).min().unwrap();
        let min_x_first_row = s
            .iter()
            .filter(|(_, y)| y == min_y)
            .map(|(x, _)| x)
            .min()
            .unwrap();
        FixedPentomino {
            squares: s.map(|(x, y)| (x - min_x_first_row, y - min_y)),
        }
    }
}

struct Pentomino {
    colour: (u8, u8, u8),
    shapes: Vec<FixedPentomino>,
}

impl Pentomino {
    fn new(colour: (u8, u8, u8), squares: [(i32, i32); 5]) -> Self {
        let fixed = FixedPentomino { squares };
        Pentomino {
            colour,
            shapes: Vec::from_iter((0..8).map(|x| fixed.transform(x)).collect::<IndexSet<_>>()),
        }
    }
}

impl fmt::Display for Pentomino {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..5 {
            writeln!(f)?;
            let mut col = 0;
            for (i, fixed) in self.shapes.iter().enumerate() {
                let min_x = fixed.squares.iter().map(|(x, _)| x).min().unwrap();
                for (x, y) in fixed.squares {
                    if y != row {
                        continue;
                    }
                    let tot_x = 6 * i as i32 + x - min_x;
                    for _ in 0..(tot_x - col) {
                        write!(f, "  ")?;
                    }
                    col = tot_x + 1;
                    let (r, g, b) = self.colour;
                    write!(f, "{}", RGB(r, g, b).paint("\u{2588}\u{2588}"))?;
                }
            }
        }
        Ok(())
    }
}

struct Board<'a> {
    pentominos: Vec<((i32, i32), &'a Pentomino, usize)>,
}

impl<'a> fmt::Display for Board<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        write!(f, " ")?;
        for _ in 0..10 {
            write!(f, "\u{2581}\u{2581}")?;
        }
        writeln!(f, " ")?;

        for row in 0..6 {
            write!(f, "\u{2595}")?;
            'col: for col in 0..10 {
                for ((fx, fy), pentomino, orientation) in &self.pentominos {
                    for (x, y) in pentomino.shapes[*orientation].squares {
                        if fy + y == row && fx + x == col {
                            let (r, g, b) = pentomino.colour;
                            write!(f, "{}", RGB(r, g, b).paint("\u{2588}\u{2588}"))?;
                            continue 'col;
                        }
                    }
                }
                write!(f, "  ")?;
            }
            writeln!(f, "\u{258f}")?;
        }

        write!(f, " ")?;
        for _ in 0..10 {
            write!(f, "\u{2594}\u{2594}")?;
        }
        writeln!(f, " ")?;

        Ok(())
    }
}

fn main() {
    let pentominos = vec![
        Pentomino::new((221, 187, 153), [(1, 0), (2, 0), (0, 1), (1, 1), (1, 2)]),
        Pentomino::new((238, 170, 170), [(0, 0), (0, 1), (0, 2), (0, 3), (0, 4)]),
        Pentomino::new((204, 204, 136), [(0, 0), (0, 1), (0, 2), (0, 3), (1, 3)]),
        Pentomino::new((170, 238, 170), [(1, 0), (1, 1), (0, 2), (1, 2), (0, 3)]),
        Pentomino::new((187, 221, 153), [(0, 0), (1, 0), (0, 1), (1, 1), (0, 2)]),
        Pentomino::new((153, 221, 187), [(0, 0), (1, 0), (2, 0), (1, 1), (1, 2)]),
        Pentomino::new((136, 204, 204), [(0, 0), (2, 0), (0, 1), (1, 1), (2, 1)]),
        Pentomino::new((153, 187, 221), [(0, 0), (0, 1), (0, 2), (1, 2), (2, 2)]),
        Pentomino::new((170, 170, 238), [(0, 0), (0, 1), (1, 1), (1, 2), (2, 2)]),
        Pentomino::new((187, 153, 221), [(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]),
        Pentomino::new((204, 136, 204), [(1, 0), (0, 1), (1, 1), (1, 2), (1, 3)]),
        Pentomino::new((221, 153, 187), [(0, 0), (1, 0), (1, 1), (1, 2), (2, 2)]),
    ];

    for p in &pentominos {
        println!("{}", p);
    }

    let board = Board {
        pentominos: vec![
            ((0, 0), &pentominos[1], 0),
            ((1, 0), &pentominos[4], 0),
            ((3, 0), &pentominos[10], 3),
            ((7, 0), &pentominos[7], 2),
            ((3, 1), &pentominos[9], 0),
            ((5, 1), &pentominos[2], 7),
            ((5, 2), &pentominos[0], 1),
            ((6, 2), &pentominos[11], 0),
            ((1, 3), &pentominos[5], 2),
            ((2, 3), &pentominos[8], 0),
            ((8, 3), &pentominos[6], 3),
            ((4, 4), &pentominos[3], 1),
        ],
    };
    println!("{}", board);
}
