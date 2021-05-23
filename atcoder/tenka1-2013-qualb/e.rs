#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

const INF: i64 = 1_000_000_000_000;

fn sqr(x: i64) -> i64 { x * x }

fn takuya(pts: &[(i64, i64)]) -> f64 {
    let n = pts.len();
    for i in 0..n {
        let (x, y) = pts[i];
        assert!(0 <= x && x <= 10000);
        assert!(0 <= y && y <= 10000);
    }
    let mut sum = 0.0;
    let mut used = vec![false; n];
    used[0] = true;
    let mut cur = pts[0];
    let mut rem = n - 1;
    while rem > 0 {
        let mut mi = (INF, 0);
        for i in 0..n {
            if !used[i] {
                mi.chmin((sqr(pts[i].0 - cur.0) + sqr(pts[i].1 - cur.1), i));
            }
        }
        let idx = mi.1;
        used[idx] = true;
        sum += (mi.0 as f64).sqrt();
        cur = pts[idx];
        rem -= 1;
    }
    sum
}

fn takahashi(pts: &[(i64, i64)]) -> f64 {
    let n = pts.len();
    let mut sum = 0.0;
    let mut used = vec![false; n];
    used[0] = true;
    let mut cur = pts[0];
    let mut rem = n - 1;
    while rem > 0 {
        let mut seq = vec![];
        for i in 0..n {
            if !used[i] {
                seq.push((sqr(pts[i].0 - cur.0) + sqr(pts[i].1 - cur.1), i));
            }
        }
        seq.sort();
        let targ = if rem >= 3 { 1 } else { 0 };
        let idx = seq[targ].1;
        used[idx] = true;
        sum += (seq[targ].0 as f64).sqrt();
        cur = pts[idx];
        rem -= 1;
    }
    sum
}

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    #[allow(unused)]
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    let n = 19;
    let mut ans = vec![(0, 0); 5];
    for i in 0..4 {
        ans[i].0 = 1000 << i;
        ans[i].1 = 1;
    }
    for i in 0..n - 5 {
        ans.insert(i, (1000, 1 + (1 << i)));
    }
    let fst = takuya(&ans);
    let snd = takahashi(&ans);
    eprintln!("takuya: {}", fst);
    eprintln!("takahashi: {}", snd);
    puts!("{}\n", ans.len());
    for &(x, y) in &ans {
        puts!("{} {}\n", x, y);
    }
}
