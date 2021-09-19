use std::cmp::*;
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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

const B: usize = 640;

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
    input! {
        n: usize, m: usize, q: usize,
        ab: [(usize1, usize1); m],
        x: [usize1; q],
    }
    let mut g = vec![vec![]; n];
    for &(a, b) in &ab {
        g[a].push(b);
        g[b].push(a);
    }
    let mut sm = vec![vec![]; n];
    let mut big = vec![vec![]; n];
    for i in 0..n {
        for &w in &g[i] {
            if g[w].len() >= B {
                big[i].push(w);
            } else {
                sm[i].push(w);
            }
        }
    }
    let mut val = vec![(0, 0); n];
    let mut lazy = vec![(0, 0); n];
    for i in 0..n {
        val[i] = (0, i as i32 + 1);
    }
    for i in 0..q {
        let x = x[i];
        let mut v = val[x];
        for &b in &big[x] {
            v.chmax(lazy[b]);
        }
        v.0 = i + 1;
        if g[x].len() >= B {
            lazy[x] = v;
        } else {
            for &w in &g[x] {
                val[w] = v;
            }
        }
    }
    let mut ans = vec![0; n];
    for i in 0..n {
        let mut v = val[i];
        for &b in &big[i] {
            v.chmax(lazy[b]);
        }
        ans[i] = v.1;
    }
    putvec!(ans);
}
