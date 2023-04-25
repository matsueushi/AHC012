#![allow(unused_variables)]
#![allow(dead_code)]

use proconio::{input, source::Source};
use rand::distributions::Uniform;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::io::BufRead;
use std::iter;

const D_MAX: usize = 10;
const L: i64 = 1_000_000_000;

const START_TEMP: i64 = 500;
const END_TEMP: i64 = 10;

fn ann_temp(t: f32) -> f32 {
    START_TEMP as f32 + (END_TEMP - START_TEMP) as f32 * t
}

fn ann_prob(best_score: usize, next_score: usize, temp: f32) -> f32 {
    ((next_score as f32 - best_score as f32) / temp).exp()
}

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
        self.lines.push((x, -L, (x + 1).min(L), L));
    }

    fn add_horizontal(&mut self, y: i64) {
        self.k += 1;
        self.lines.push((-L, y, L, (y + 1).min(L)));
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

struct Pieces {
    b: [usize; 10],
}

impl Pieces {
    fn score(&self, input: &Input) -> usize {
        let asum = input.a.iter().sum::<usize>();
        let mut minabsum = 0;
        for i in 0..D_MAX {
            minabsum += input.a[i].min(self.b[i]);
        }
        let score = (1_000_000.0 * (minabsum as f64 / asum as f64)).round() as usize;
        score
    }
}

// カットの情報
#[derive(Debug, Clone)]
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

    // スコアを計算したい
    fn pieces(&self, cake: &Cake) -> Pieces {
        let ub = cake.xs.len() - 1;
        let uus = iter::once(&0)
            .chain(&self.us)
            .chain(iter::once(&ub))
            .collect::<Vec<_>>();

        let vb = cake.ys.len() - 1;
        let vvs = iter::once(&0)
            .chain(&self.vs)
            .chain(iter::once(&vb))
            .collect::<Vec<_>>();

        // println!("{:?}", uus);
        // println!("{:?}", vvs);

        let mut b = [0; 10];
        for u in uus.windows(2) {
            for v in vvs.windows(2) {
                // println!("{:?} {:?}", u, v);
                let (&u0, &u1) = (u[0], u[1]);
                let (&v0, &v1) = (v[0], v[1]);
                let c = cake.count(u0, u1, v0, v1);
                // println!("{}", c);
                if c >= 1 && c <= D_MAX {
                    b[c - 1] += 1;
                }
            }
        }

        Pieces { b }
    }

    fn random_move(target: &mut Vec<usize>, right: usize, r: usize, rng: &mut ChaCha8Rng) {
        let orig_pos = target[r];

        let new_pos = if r == 0 {
            rng.gen_range(target[0], target[1])
        } else if r == target.len() - 1 {
            rng.gen_range(target[r - 1] + 1, target[r] + 1)
        } else {
            // eprintln!("{:?} {} {}", target, target[r - 1] + 1, target[r + 1]);
            rng.gen_range(target[r - 1] + 1, target[r + 1])
        };
        target[r] = new_pos;
    }

    fn random_move_x(&mut self, cake: &Cake, r: usize, rng: &mut ChaCha8Rng) {
        Self::random_move(&mut self.us, cake.xs.len(), r, rng);
    }

    fn random_move_y(&mut self, cake: &Cake, r: usize, rng: &mut ChaCha8Rng) {
        Self::random_move(&mut self.vs, cake.ys.len(), r, rng);
    }
}

// 座標圧縮して使いやすい状態になっているケーキ
#[derive(Debug)]
struct Cake {
    xs: Vec<i64>,        // 圧縮後のx座標
    ys: Vec<i64>,        // 圧縮後のy座標
    cs: Vec<Vec<usize>>, // 累積されている数
}

impl Cake {
    fn new(input: &Input) -> Self {
        let mut xs = vec![-L];
        let mut ys = vec![-L];
        for &(x, y) in &input.xy {
            xs.push(x);
            ys.push(y);
        }
        xs.push(L);
        ys.push(L);
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

        let mut cs = vec![vec![0; m]; n];
        // x 方向
        for i in 0..n {
            for j in 1..m {
                cs[i][j] = cs[i][j - 1] + zs[i][j];
            }
        }
        for j in 0..m {
            for i in 1..n {
                cs[i][j] += cs[i - 1][j];
            }
        }

        Self { xs, ys, cs }
    }

    fn count(&self, u0: usize, u1: usize, v0: usize, v1: usize) -> usize {
        self.cs[u1][v1] - self.cs[u1][v0] - self.cs[u0][v1] + self.cs[u0][v0]
    }
}

pub fn solve(input: &Input) {
    let since = std::time::Instant::now();
    eprintln!("round,time,score");

    let cake = Cake::new(&input);

    let mut round = 0;

    let mut best_cut = Cut {
        us: Vec::new(),
        vs: Vec::new(),
    };
    let mut best_score = 0;

    // ランダムシード
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0);

    for i in 20..=100 {
        round += 1;
        let k = i;
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
        let pieces = cut.pieces(&cake);
        let score = pieces.score(input);

        if score > best_score {
            best_score = score;
            best_cut = cut;
            // let t = since.elapsed().as_secs_f32();
            // log_score(round, t, best_score);
            // println!("{}", best_cut.lines(&cake));
        }
    }

    // 一つづつ動かす
    let uniform = Uniform::new(0.0f32, 1.0f32);

    for i in 0..10000 {
        round += 1;

        let mut new_cut = best_cut.clone();
        // 適当に縦の線を一個選んで動かす
        if i % 2 == 0 {
            let r = rng.gen_range(0, new_cut.us.len());
            new_cut.random_move_x(&cake, r, &mut rng);
        } else {
            let r = rng.gen_range(0, new_cut.vs.len());
            new_cut.random_move_y(&cake, r, &mut rng);
        };

        let pieces = new_cut.pieces(&cake);
        let score = pieces.score(input);

        // 焼きなまし
        let temp = ann_temp(i as f32 / 10000f32);
        let threshold = ann_prob(score, best_score, temp);
        let prob = rng.sample(uniform);
        println!("{} {}", prob, threshold);

        if score > best_score || prob > threshold {
            best_cut = new_cut;
            best_score = score;
            let t = since.elapsed().as_secs_f32();
            log_score(round, t, score);
            // println!("{}", best_cut.lines(&cake));
            // eprintln!("{:?}", best_cut.us);
        }
    }
}

#[cfg(feature = "local")]
fn log_score(round: i32, t: f32, score: usize) {
    eprintln!("{},{},{}", round, t, score);
}

#[cfg(not(feature = "local"))]
fn log_score(round: i32, t: f32, score: usize) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cake() {
        let input = Input {
            n: 1 * 1 + 3 * 1,
            k: 10,
            a: vec![1, 0, 1, 0, 0, 0, 0, 0, 0, 0],
            xy: vec![(1000, 1000), (2000, 2000), (1000, 2000), (3000, 3000)],
        };
        println!("input:");
        println!("{}", input);

        let cake = Cake::new(&input);
        println!("{:?}", cake);

        let cut = Cut {
            us: vec![1],
            vs: vec![1],
        };

        let lines = cut.lines(&cake);
        println!("{}", lines);

        let pieces = cut.pieces(&cake);
        let score = pieces.score(&input);
        println!("{}", score);
    }
}
