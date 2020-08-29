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
    ($next:expr, [graph1; $len:expr]) => {{
        let mut g = vec![vec![]; $len];
        let ab = read_value!($next, [(usize1, usize1)]);
        for (a, b) in ab {
            g[a].push(b);
            g[b].push(a);
        }
        g
    }};
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };
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

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

fn cost(w: usize, dpval: usize, st: usize, len: usize) -> usize {
    if dpval >= w {
        1 << 30
    } else {
        dpval - st + 1 - len
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        h: usize, w: usize,
        ab: [(usize1, usize1); h],
    }
    // (dp, st, len)
    let mut dp = BTreeSet::new();
    // (val, dp, st, len)
    let mut vals = BTreeSet::new();
    for i in 0..w {
        dp.insert((i, i, 1));
        vals.insert((0, i, i, 1));
    }
    for i in 0..h {
        let (a, b) = ab[i];
        let rng = dp.range((a, 0, 0)..(b + 1, 0, 0));
        let rng: Vec<_> = rng.into_iter().cloned().collect();
        let mut mist = w;
        let mut maen = 0;
        for ent in rng {
            let (dpval, st, len) = ent;
            dp.remove(&ent);
            vals.remove(&(cost(w, dpval, st, len), dpval, st, len));
            mist = min(mist, st);
            maen = max(maen, st + len);
        }
        if mist < maen {
            dp.insert((b + 1, mist, maen - mist));
            vals.insert((cost(w, b + 1, mist, maen - mist), b + 1, mist, maen - mist));
        }
        let &(mi, _, _, _) = vals.iter().next().unwrap();
        if mi >= w {
            puts!("-1\n");
        } else {
            puts!("{}\n", mi + i + 1);
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
