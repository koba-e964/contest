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
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    }};

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

const MOD: i64 = 1_000_000_007;

fn make_period(x: &[i64], len: usize) -> Vec<i64> {
    let mut ret = vec![0; len];
    for i in 0..x.len() {
        ret[i % len] += x[i];
        ret[i % len] %= MOD;
    }
    ret
}

const MAXLEN: usize = 200000;

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        k: [[i64]; n],
        q: usize,
        xy: [(usize, usize); q],
    }
    let mut xy = xy;
    let mut ans = vec![0; q];
    let mut bucket = vec![vec![]; MAXLEN + 1];
    for i in 0..q {
        if k[xy[i].0].len() > k[xy[i].1].len() {
            let e = &mut xy[i];
            std::mem::swap(&mut e.0, &mut e.1);
        }
        let len = k[xy[i].0].len();
        bucket[len].push(i);
    }
    let mut memo = HashMap::new();
    for len in 1..MAXLEN + 1 {
        if bucket[len].is_empty() { continue; }
        let mut per = vec![vec![]; n];
        for &idx in &bucket[len] {
            let (x, y) = xy[idx];
            assert_eq!(k[x].len(), len);
            if let Some(&result) = memo.get(&(x, y)) {
                ans[idx] = result;
                continue;
            }
            if per[y].is_empty() { per[y] = make_period(&k[y], len); }
            let mut tot = 0;
            for i in 0..len {
                tot += k[x][i] * per[y][i];
                tot %= MOD;
            }
            ans[idx] = tot;
            memo.insert((x, y), tot);
        }
    }
    for ans in ans {
        puts!("{}\n", ans);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
