#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes.by_ref().map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr,) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

type Coord = i64; // the type of coordinates
type P = (Coord, Coord);

fn det((ax, ay): P, (bx, by): P) -> Coord {
    ax * by - ay * bx
}

fn sub((ax, ay): P, (bx, by): P) -> P {
    (ax - bx, ay - by)
}

fn convex_hull(ps: &[P]) -> Vec<P> {
    let n = ps.len();
    let mut k = 0;
    let mut ps = ps.to_vec();
    ps.sort();
    let mut ch = vec![(0, 0); 2 * n];
    // lower
    for i in 0..n {
        while k >= 2 &&
            det(sub(ps[i], ch[k - 2]), sub(ch[k - 1], ch[k - 2])) >= 0 {
            k -= 1;
        }
        ch[k] = ps[i];
        k += 1;
    }
    // upper
    let t = k + 1;
    for i in (0..n - 1).rev() {
        while k >= t &&
            det(sub(ps[i], ch[k - 2]), sub(ch[k - 1], ch[k - 2])) >= 0 {
            k -= 1;
        }
        ch[k] = ps[i];
        k += 1;
    }
    ch.truncate(k - 1);
    ch
}

fn solve() {
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
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }
    let mut hm = HashMap::new();
    for i in 0..n {
        hm.insert(xy[i], i);
    }
    let ch = convex_hull(&xy);
    let m = ch.len();
    let mut div = vec![0.0; m];
    for i in 0..m {
        let (dx, dy) = sub(ch[(i + 1) % m], ch[i]);
        div[i] = (dy as f64).atan2(dx as f64);
    }
    let mut ans = vec![0.0; n];
    let pi = std::f64::consts::PI;
    for i in 0..m {
        let idx = hm[&ch[i]];
        let mut diff = div[i] - div[(i + m - 1) % m];
        if diff < 0.0 {
            diff += 2.0 * pi;
        }
        ans[idx] = diff / (2.0 * pi);
    }
    for i in 0..n {
        puts!("{}\n", ans[i]);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
