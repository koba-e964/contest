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

// This solution was written after the author read the editorial.
fn rec(a: Vec<i32>) -> bool {
    if a.len() <= 1 {
        return true;
    }
    let mi = a[0];
    let n = a.len();
    for i in 0..n {
        if a[i] < mi {
            return false;
        }
    }
    let mut cluster = vec![];
    let mut cur = vec![mi];
    for i in 1..n {
        if a[i] == mi && a[i - 1] != mi {
            // separate
            cluster.push(cur);
            cur = vec![];
        }
        cur.push(a[i]);
    }
    cluster.push(cur);
    let mut pool = cluster.clone();
    pool.sort();
    pool.dedup();
    // Two elements may compare equal.
    // So we need to dedup before this sorting.
    pool.sort_by(|x, y| {
        let mut fst = x.to_vec();
        fst.extend_from_slice(y);
        let mut snd = y.to_vec();
        snd.extend_from_slice(x);
        fst.cmp(&snd)
    });
    let mut inv = HashMap::new();
    for i in 0..pool.len() {
        inv.insert(pool[i].clone(), i as i32);
    }
    let mapped = cluster.into_iter().map(|c| inv[&c]).collect();
    rec(mapped)
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        s: chars,
    }
    let vec = s.into_iter().map(|ch| ch as i32).collect();
    puts!("{}\n", if rec(vec) { "Yes" } else { "No" });
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
