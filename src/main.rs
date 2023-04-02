use clap::Parser;
use pentominos::args::*;

use ansi_term::Colour::RGB;
use indexmap::IndexSet;
use std::fmt;

const BOARD_SIZE_X: usize = 10;
const BOARD_SIZE_Y: usize = 6;

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
    grid_cache: [[bool; BOARD_SIZE_X + 7]; BOARD_SIZE_Y + 4],
}

impl<'a> Board<'a> {
    fn new() -> Self {
        let mut grid_cache = [[false; BOARD_SIZE_X + 7]; BOARD_SIZE_Y + 4];
        for row in grid_cache.iter_mut().take(BOARD_SIZE_Y) {
            for s in row.iter_mut().take(3) {
                *s = true;
            }
            for s in row.iter_mut().skip(BOARD_SIZE_X + 3) {
                *s = true;
            }
        }
        for row in grid_cache.iter_mut().skip(BOARD_SIZE_Y) {
            *row = [true; BOARD_SIZE_X + 7];
        }
        Board {
            pentominos: Vec::new(),
            grid_cache,
        }
    }

    fn push(
        &mut self,
        (fx, fy): (i32, i32),
        pentomino: &'a Pentomino,
        orientation: usize,
    ) -> Result<(), ()> {
        for (x, y) in pentomino.shapes[orientation].squares {
            let x = fx + x + 3;
            let y = fy + y;

            if self.grid_cache[y as usize][x as usize] {
                return Err(());
            }
        }
        self.update_cache((fx, fy), pentomino, orientation, true);
        self.pentominos.push(((fx, fy), pentomino, orientation));
        Ok(())
    }

    fn pop(&mut self) {
        let (xy, pentomino, orientation) = self.pentominos.pop().unwrap();
        self.update_cache(xy, pentomino, orientation, false);
    }

    fn update_cache(
        &mut self,
        (fx, fy): (i32, i32),
        pentomino: &Pentomino,
        orientation: usize,
        value: bool,
    ) {
        for (x, y) in pentomino.shapes[orientation].squares {
            self.grid_cache[(fy + y) as usize][(fx + x + 3) as usize] = value;
        }
    }

    fn next_free_square_from(&self, x: i32, y: i32) -> Option<(i32, i32)> {
        for n in (x as usize + y as usize * BOARD_SIZE_X)..(BOARD_SIZE_X * BOARD_SIZE_Y) {
            let x = n % BOARD_SIZE_X;
            let y = n / BOARD_SIZE_X;
            if !self.grid_cache[y][x + 3] {
                return Some((x as i32, y as i32));
            }
        }
        None
    }
}

impl fmt::Display for Board<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f)?;
        write!(f, " ")?;
        for _ in 0..BOARD_SIZE_X {
            write!(f, "\u{2581}\u{2581}")?;
        }
        writeln!(f, " ")?;

        for row in 0..BOARD_SIZE_Y as i32 {
            write!(f, "\u{2595}")?;
            'col: for col in 0..BOARD_SIZE_X as i32 {
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
        for _ in 0..BOARD_SIZE_X {
            write!(f, "\u{2594}\u{2594}")?;
        }
        writeln!(f, " ")?;

        Ok(())
    }
}

fn solve_recursively<'a>(
    board: &mut Board<'a>,
    (x, y): (i32, i32),
    pentominos: &mut Vec<&'a Pentomino>,
    num_solutions: &mut i32,
    print_board: bool,
) {
    //println!("{}", board);
    for i in 0..pentominos.len() {
        let pentomino = pentominos.remove(i);
        for orientation in 0..pentomino.shapes.len() {
            if board.push((x, y), pentomino, orientation).is_err() {
                continue;
            }
            if let Some(xy) = board.next_free_square_from(x, y) {
                solve_recursively(board, xy, pentominos, num_solutions, print_board);
            } else {
                if print_board {
                    println!("{}", board);
                };
                *num_solutions += 1;
                board.pop();
                pentominos.insert(i, pentomino);
                // panic!();
                return;
            }
            board.pop();
        }
        pentominos.insert(i, pentomino);
    }
}

fn main() {
    // Parse command line arguments
    let args = Args::parse();

    // Definition of all pieces
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

    // Print individual pieces
    if args.pieces {
        pentominos.iter().for_each(|p| println!("{}", &p));
    }

    // Solve board and optionally print solutions
    if args.solve || args.count {
        let mut pr = pentominos.iter().collect();
        let mut num_solutions = 0;
        solve_recursively(
            &mut Board::new(),
            (0, 0),
            &mut pr,
            &mut num_solutions,
            args.solve,
        );
        // Print number of solutions
        if args.count {
            println!("Found {} solutions.", num_solutions);
        }
    }
}
