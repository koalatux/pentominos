use std::fmt;

#[derive(Debug)]
struct Pentomino {
    squares: [(i32, i32); 5],
}

impl Pentomino {
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

        let min_x = s.iter().map(|(x, _)| x).min().unwrap();
        let min_y = s.iter().map(|(_, y)| y).min().unwrap();

        Pentomino {
            squares: s.map(|(x, y)| (x - min_x, y - min_y)),
        }
    }
}

impl fmt::Display for Pentomino {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut col = 0;
        let mut row = 0;
        writeln!(f)?;
        for (x, y) in self.squares {
            if row != y {
                col = 0;
                row = y;
                writeln!(f)?;
            }
            for _ in 0..(x - col) {
                write!(f, "  ")?;
            }
            col = x + 1;
            write!(f, "\u{2588}\u{2588}")?;
        }
        Ok(())
    }
}

fn main() {
    let f = Pentomino {
        squares: [(1, 0), (2, 0), (0, 1), (1, 1), (1, 2)],
    };

    println!("f: {}", f);
    println!();
    for i in 0..8 {
        println!("f {}: {}", i, f.transform(i));
    }
}
