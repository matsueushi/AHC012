use proconio::{input, source::Source};
use std::io::BufRead;

const D_MAX: usize = 10;
const R: i64 = 10000;
const L: i64 = 1_000_000_000;

// 入力

#[derive(Clone, Debug)]
pub struct Input {
    pub n: usize,
    pub k: usize,
    pub a: Vec<usize>,
    pub xy: Vec<(i64, i64)>,
}

impl Input {
    pub fn from_source<R: BufRead, S: Source<R>>(mut source: &mut S) -> Self {
        input! {
            from &mut source,
            n: usize,
            k: usize,
            a: [usize; D_MAX],
            xy: [(i64, i64); n],
        }
        Self { n, k, a, xy }
    }
}

// ロジック

// 座標圧縮
// pub fn compress_coord(coords: &[(i64, i64)]) {
//     eprintln!("{:?}", coords);
// }

type Line = (i64, i64, i64, i64);

struct Cut {
    k: usize,
    lines: Vec<Line>,
}

impl Cut {
    fn new() -> Self {
        Self {
            k: 0,
            lines: Vec::new(),
        }
    }

    fn add_vertical(&mut self, x: i64) {
        self.k += 1;
        self.lines.push((x, -L, x + 1, L));
    }

    fn add_horizontal(&mut self, y: i64) {
        self.k += 1;
        self.lines.push((-L, y, L, y + 1));
    }
}

impl std::fmt::Display for Cut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.k)?;
        for line in &self.lines {
            println!("{} {} {} {}", line.0, line.1, line.2, line.3);
        }
        Ok(())
    }
}

pub fn solve(input: &Input) {
    // compress_coord(&input.xy);

    let k = input.k;
    let d = R as usize / k;
    let mut cut = Cut::new();
    for x in (-R..R).step_by(4 * d) {
        cut.add_vertical(x);
    }
    for y in (-R..R).step_by(4 * d) {
        cut.add_horizontal(y);
    }
    println!("{}", cut);
}
