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
        l: f64,
        tv: [(f64, f64); n],
    }
    // Let x be the sum of v for all entry (v, _, _).
    // An entry (v, p, acc) means
    // (x - v, acc - p) and (x, acc) are connected by a segment.
    // We are to find max_y{(f(x) - f(x - y)) / y}.
    let mut pool: Vec<(f64, f64, f64)> = Vec::new();
    // Sentinel. l liter of water with temperature -1.0
    pool.push((l, -1.0, -l));
    // The pointer is at pool[cur].0 + pos
    // This pointer should always be at x - l, where x is described above.
    let mut cur = 0;
    let mut pos = 0.0;
    for (t, v) in tv {
        let mut last = pool[pool.len() - 1].2;
        pool.push((v, t * v, last + t * v));
        // Advances pointer by v
        let mut rem = v;
        while rem > 0.0 {
            if pool[cur].0 - pos < rem {
                rem -= pool[cur].0 - pos;
                cur += 1;
                pos = 0.0;
            } else {
                pos += rem;
                break;
            }
        }
        assert!(pos <= pool[cur].0 + 1e-9);
        // Greedy chmax
        while pool.len() >= 2 {
            let (v1, p1, acc1) = pool[pool.len() - 1];
            if v1 >= l { break; }
            let (v2, p2, _acc2) = pool[pool.len() - 2];
            if p2 / v2 > p1 / v1 {
                pool.pop();
                pool.pop();
                if v1 + v2 >= l {
                    pool.clear();
                    let newp = p1 + p2 / v2 * (l - v1);
                    pool.push((l, newp, newp));
                    // x = l. Pointer's position is reset to 0.0.
                    cur = 0;
                    pos = 0.0;
                } else {
                    pool.push((v1 + v2, p1 + p2, acc1));
                    if cur == pool.len() {
                        cur -= 1;
                        pos += pool[cur].0;
                    }
                }
            } else { break; }
        }
        let (v, p, acc) = pool[cur];
        let acclast = pool.last().unwrap().2;
        puts!("{:.15}\n", (acclast - acc + (p / v) * (v - pos)) / l);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
