#![allow(unused_variables)]
#![allow(dead_code)]

use proconio::{input, source::Source};
// use rand::{Rng, SeedableRng};
use std::io::BufRead;
use std::iter;

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

impl std::fmt::Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} {}", self.n, self.k)?;
        writeln!(
            f,
            "{}",
            self.a
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        )?;
        for &(x, y) in &self.xy {
            writeln!(f, "{} {}", x, y)?;
        }
        Ok(())
    }
}

// ロジック
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
    xs: Vec<i64>,        // 圧縮後のx座標
    ys: Vec<i64>,        // 圧縮後のy座標
    cs: Vec<Vec<usize>>, // 累積されている数
}

impl Cake {
    fn new(input: &Input) -> Self {
        let mut xs = Vec::new();
        let mut ys = Vec::new();
        for &(x, y) in &input.xy {
            xs.push(x);
            ys.push(y);
        }
        xs.sort();
        ys.sort();
        xs.dedup();
        ys.dedup();

        let n = xs.len();
        let m = ys.len();
        let mut zs = vec![vec![0; m]; n];
        for (x, y) in &input.xy {
            let i = xs.binary_search(x).unwrap();
            let j = ys.binary_search(y).unwrap();
            zs[i][j] += 1;
        }

        let mut cs = vec![vec![0; m + 1]; n + 1];
        for i in 0..n {
            for j in 0..m {
                cs[i + 1][j + 1] = cs[i + 1][j] + zs[i][j];
            }
        }
        for j in 0..m {
            for i in 0..n {
                cs[i + 1][j + 1] += cs[i][j + 1];
            }
        }

        println!("{:?}", xs);
        println!("{:?}", ys);
        Self { xs, ys, cs }
    }
}

pub fn solve(input: &Input) {
    let cake = Cake::new(&input);
    // eprintln!("{}", cake.xs.len());
    // eprintln!("{}", cake.ys.len());
    // let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0);

    let k = input.k;
    let step_x = 2 * cake.xs.len() / k;
    let step_y = 2 * cake.ys.len() / k;
    let mut us = Vec::new();
    let mut vs = Vec::new();
    for u in (step_x..cake.xs.len()).step_by(step_x) {
        us.push(u);
    }
    for v in (step_y..cake.ys.len()).step_by(step_y) {
        vs.push(v);
    }
    let cut = Cut { us, vs };
    println!("{}", cut.lines(&cake));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cake() {
        let input = Input {
            n: 1 * 1 + 2 * 1 + 3 * 1,
            k: 10,
            a: vec![1, 1, 1, 0, 0, 0, 0, 0, 0, 0],
            xy: vec![
                (1000, 1000),
                (2000, 2000),
                (1000, 2000),
                (3000, 3000),
                (4000, 4000),
                (5000, 5000),
            ],
        };
        println!("input:");
        println!("{}", input);
    }
}
