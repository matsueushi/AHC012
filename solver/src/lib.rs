use proconio::{input, source::Source};
use rand::{Rng, SeedableRng};
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

// カットした情報の出力用
struct CutLines {
    k: usize,
    lines: Vec<Line>,
}

impl CutLines {
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

impl std::fmt::Display for CutLines {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.k)?;
        for line in &self.lines {
            println!("{} {} {} {}", line.0, line.1, line.2, line.3);
        }
        Ok(())
    }
}

// カットの情報
struct Cut {
    xs: Vec<i64>,
    ys: Vec<i64>,
}

impl Cut {
    fn lines(&self) -> CutLines {
        let mut cut_lines = CutLines::new();
        for &x in &self.xs {
            cut_lines.add_vertical(x);
        }
        for &y in &self.ys {
            cut_lines.add_horizontal(y);
        }
        cut_lines
    }
}

pub fn solve(input: &Input) {
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0);

    let k = input.k;
    let d = R as usize / k;
    for _ in 0..50 {
        let mut xs = Vec::new();
        let mut ys = Vec::new();
        for x in (-R..R).step_by(4 * d) {
            let d = d as i64;
            let r = rng.gen_range(-d, d);
            xs.push(x + r);
        }
        for y in (-R..R).step_by(4 * d) {
            let d = d as i64;
            let r = rng.gen_range(-d, d);
            ys.push(y + r);
        }
        let cut = Cut { xs, ys };
        println!("{}", cut.lines());
    }
}
