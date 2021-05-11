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
        n: usize, q: usize,
        x: [i64; n],
        w: [i64; q],
    }
    let mut seg = vec![];
    for i in 0..n - 1 {
        seg.push((x[i + 1] - x[i], i, i + 1));
    }
    const INF: i64 = 1 << 60;
    seg.push((INF, n, 0));
    seg.push((INF, n - 1, n));
    seg.sort();
    let mut l = 0;
    let mut r = 0;
    let mut pos = 0;
    let mut dpl = vec![0; n + 1];
    let mut dpr = vec![0; n + 1];
    let mut imosl = vec![0; n + 1];
    let mut imosr = vec![0; n + 1];
    for &w in &w {
        l += w;
        r -= w;
        if w > 0 {
            if r < 0 {
                while seg[pos].0 <= l {
                    dpr[pos] += seg[pos].0 - (l + r);
                    pos += 1;
                }
                imosr[pos] += -r;
                r = 0;
            }
        } else {
            if l < 0 {
                while seg[pos].0 <= r {
                    dpl[pos] += seg[pos].0 - (l + r);
                    pos += 1;
                }
                imosl[pos] += -l;
                l = 0;
            }
        }
    }
    for i in 1..n + 1 {
        imosl[i] += imosl[i - 1];
        imosr[i] += imosr[i - 1];
    }
    for i in 0..n + 1 {
        dpl[i] += imosl[i];
        dpr[i] += imosr[i];
    }
    let mut ans = vec![0; n + 1];
    for i in 0..n + 1 {
        ans[seg[i].2] += dpl[i];
        ans[seg[i].1] += dpr[i];
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
