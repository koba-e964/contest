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

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        pq: [(usize1, usize1)],
        a: [usize1],
    }
    // Verification
    assert!(1 <= n && n <= 100000);
    assert!(pq.len() <= 100000);
    let mut seen = HashSet::new();
    for &(p, q) in &pq {
        assert!(p < q);
        assert_eq!(seen.get(&(p, q)), None);
        seen.insert((p, q));
    }

    // solve, O(q (m + n))
    let mut g = vec![vec![]; n];
    for &(p, q) in &pq {
        g[p].push(q);
        g[q].push(p);
    }
    for &a in &a {
        let mut que = VecDeque::new();
        que.push_back((a, 0));
        let mut vis = vec![false; n];
        let mut ma = 0;
        let mut count = 0;
        while let Some((v, d)) = que.pop_front() {
            if vis[v] { continue; }
            vis[v] = true;
            count += 1;
            ma = max(ma, d);
            for &w in &g[v] {
                que.push_back((w, d + 1));
            }
            
        }
        let mut turn = 0;
        while ma > 1 << turn {
            turn += 1;
        }
        puts!("{} {}\n", count - 1, turn);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
