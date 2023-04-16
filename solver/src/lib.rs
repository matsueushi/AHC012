use proconio::{input, source::Source};
use rand::{Rng, SeedableRng};
use std::io::BufRead;

const D_MAX: usize = 10;
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
    us: Vec<usize>, // カットする圧縮後のx座標
    vs: Vec<usize>, // カットする圧縮後のy座標
}

impl Cut {
    fn lines(&self, cake: &Cake) -> CutLines {
        let mut cut_lines = CutLines::new();
        for &u in &self.us {
            let x = cake.xs[u];
            cut_lines.add_vertical(x);
        }
        for &v in &self.vs {
            let y = cake.ys[v];
            cut_lines.add_horizontal(y);
        }
        cut_lines
    }
}

// 座標圧縮して使いやすい状態になっているケーキ
struct Cake {
    xs: Vec<i64>, // 圧縮後のx座標
    ys: Vec<i64>, // 圧縮後のy座標
}

impl Cake {
    fn new(input: &Input) -> Self {
        let mut xs = Vec::new();
        let mut ys = Vec::new();
        for &(x, y) in &input.xy {
            xs.push(x);
            ys.push(y);
        }
        Self { xs, ys }
    }
}

pub fn solve(input: &Input) {
    let cake = Cake::new(&input);

    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0);

    let k = input.k;
    let step_x = 2 * cake.xs.len() / k;
    let step_y = 2 * cake.ys.len() / k;
    for _ in 0..50 {
        let mut us = Vec::new();
        let mut vs = Vec::new();
        for u in (0..cake.xs.len() - step_x).step_by(step_x) {
            let r = rng.gen_range(0, step_x / 2);
            us.push(u + r);
        }
        for v in (0..cake.ys.len() - step_y).step_by(step_y) {
            let r = rng.gen_range(0, step_y / 2);
            vs.push(v + r);
        }
        let cut = Cut { us, vs };
        println!("{}", cut.lines(&cake));
    }
}
