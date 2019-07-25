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

fn search(g: &[Vec<(i64, usize)>], k: usize, x: i64) -> bool {
    let k = 2 * k;
    let mut cnt = 0;
    let n = g.len();
    for i in 0..n {
        let mut que = BinaryHeap::new();
        que.push((0, i));
        let mut dm = HashMap::new();
        while let Some((d, v)) = que.pop() {
            let d = -d;
            if dm.contains_key(&v) {
                continue;
            } else {
                dm.insert(v, d);
                if v != i {
                    cnt += 1;
                }
                if cnt >= k {
                    return true;
                }
            }
            for &(c, w) in &g[v] {
                if d + c > x {
                    break;
                }
                que.push((-(d + c), w));
            }
        }
    }
    cnt >= k
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize, m: usize, k: usize,
        xyw: [(usize1, usize1, i64); m],
    }
    let mut g = vec![vec![]; n];
    for &(x, y, w) in &xyw {
        g[x].push((w, y));
        g[y].push((w, x));
    }
    for i in 0..n {
        g[i].sort();
    }
    let mut pass: i64 = 1 << 50;
    let mut fail = 0;
    while pass - fail > 1 {
        let mid = (pass + fail) / 2;
        if search(&g, k, mid) {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    puts!("{}\n", pass);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
