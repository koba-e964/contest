#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
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

const INF: i64 = 1 << 50;

fn calc(b: &[Vec<i64>]) -> i64 {
    let h = b.len();
    let w = b[0].len();
    let mut acc = vec![vec![0; w + 1]; h];
    for i in 0..h {
        for j in 0..w {
            acc[i][j + 1] = acc[i][j] + b[i][j];
        }
    }
    let mut ma = -INF;
    let mut gl = vec![-INF; h + 1];
    let mut gr = vec![-INF; h + 1];
    for i in 0..w {
        for j in i + 1..w + 1 {
            let mut dpl = vec![0; h + 1];
            let mut dpr = vec![0; h + 1];
            let mut mi = INF;
            let mut sum = 0;
            for k in 0..h + 1 {
                dpl[k] = sum - mi;
                mi.chmin(sum);
                if k < h {
                    sum += acc[k][j] - acc[k][i];
                }
            }
            mi = INF;
            sum = 0;
            for k in (0..h + 1).rev() {
                dpr[k] = sum - mi;
                mi.chmin(sum);
                if k > 0 {
                    sum += acc[k - 1][j] - acc[k - 1][i];
                }
            }
            for k in 0..h + 1 {
                gl[k].chmax(dpl[k]);
                gr[k].chmax(dpr[k]);
            }
        }
    }
    for k in (0..h).rev() {
        gr[k] = max(gr[k], gr[k + 1]);
        ma.chmax(gr[k] + gl[k]);
    }
    ma
}

fn main() {
    input! {
        h: usize, w: usize,
        b: [[i64; w]; h],
    }
    let mut ma = calc(&b);
    let mut c = vec![vec![0; h]; w];
    for i in 0..h {
        for j in 0..w {
            c[j][i] = b[i][j];
        }
    }
    ma.chmax(calc(&c));
    println!("{}", ma);
}
