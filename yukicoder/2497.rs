use std::cmp::*;
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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn gcd(mut x: i64, mut y: i64) -> i64 {
    while y != 0 {
        let r = x % y;
        x = y; y = r;
    }
    x.abs()
}

const MOD: i64 = 998_244_353;

// https://yukicoder.me/problems/no/2497 (3)
// A が素数冪だとすると、この問題は (min, max)-semiring 上の最短路問題であり O(M log N)-time で解ける。
// A[i] を素因数分解すればこれは解ける。A[i] の素因数の個数は高々 9 個なので、登場する素因数は 9N 個程度であり
// 実行時間に不安が残る。素因数分解にこだわらず、
// gcd によって適切な因子基底を求めることにすれば、因数の個数をもっと少なくできるはず。
// Tags: factor-base, single-source-shortest-path
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, m: usize,
        a: [i64; n],
        uv: [(usize1, usize1); m],
    }
    let mut g = vec![vec![]; n];
    for &(u, v) in &uv {
        g[u].push(v);
        g[v].push(u);
    }
    let mut factors = a.clone();
    loop {
        let mut new = vec![];
        let mut changed = false;
        for i in 0..factors.len() {
            for j in 0..i {
                let g = gcd(factors[i], factors[j]);
                if g != 1 {
                    factors[i] /= g;
                    factors[j] /= g;
                    new.push(g);
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
        for &f in &factors {
            if f != 1 {
                new.push(f);
            }
        }
        new.sort(); new.dedup();
        factors = new;
    }
    eprintln!("f = {:?}", factors);
    let mut ans = vec![1; n];
    for &fac in &factors {
        let mut e = vec![0; n];
        for i in 0..n {
            let mut v = a[i];
            while v % fac == 0 {
                v /= fac;
                e[i] += 1;
            }
        }
        let mut que = BinaryHeap::new();
        que.push((Reverse(e[0]), 0));
        const INF: u32 = 1 << 28;
        let mut dist = vec![INF; n];
        while let Some((Reverse(d), v)) = que.pop() {
            if dist[v] <= d { continue; }
            dist[v] = d;
            for &w in &g[v] {
                let nd = max(d, e[w]);
                que.push((Reverse(nd), w));
            }
        }
        for i in 0..n {
            for _ in 0..dist[i] {
                ans[i] = ans[i] * fac % MOD;
            }
        }
    }
    for a in ans {
        puts!("{}\n", a);
    }
}
