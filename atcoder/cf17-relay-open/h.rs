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

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

// The author read the editorial.
const INF: i64 = 1 << 40;

fn dfs(small: &[(usize, usize)], large: &[usize],
       mx: &[(Vec<usize>, Option<usize>, i64)],
       v: usize, path: &mut Vec<usize>) -> i64 {
    if v >= small.len() {
        let m = large.len();
        let mut profit = vec![0; m];
        let mut tot = 0;
        for &(ref s, b, x) in mx {
            let surpass = (0..small.len()).all(|i| path[i] >= s[i]);
            if !surpass { continue; }
            if let Some(b) = b {
                profit[b] += x;
            } else {
                tot += x;
            }
        }
        for i in 0..m {
            tot += max(0, profit[i]);
        }
        return tot;
    }
    let mut ma = -INF;
    for i in 0..small[v].1 + 1 {
        path.push(i);
        let tmp = dfs(small, large, mx, v + 1, path);
        ma = max(ma, tmp);
        path.pop();
    }
    ma
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        q: usize,
        mx: [(usize, i64); q],
    }
    let small = vec![
        (2, 8),
        (3, 5),
        (5, 3),
        (7, 2),
        (11, 2),
        (13, 2),
        (17, 2),
    ];
    let large = {
        let mut pr = vec![true; 301];
        pr[0] = false;
        pr[1] = false;
        for i in 2..301 {
            if !pr[i] { continue; }
            for j in 2..300 / i + 1 {
                pr[i * j] = false;
            }
        }
        let prs: Vec<_> = (2..301).filter(|&i| pr[i]).collect();
        prs
    };
    let mx: Vec<(Vec<usize>, Option<usize>, i64)> =
        mx.into_iter().map(|(m, x)| {
            let mut v = m;
            let mut s = vec![0; small.len()];
            for i in 0..small.len() {
                let p = small[i].0;
                while v % p == 0 {
                    v /= p;
                    s[i] += 1;
                }
            }
            let b = large.iter().position(|&x| x == v);
            (s, b, x)
        }).collect();
    let ma = dfs(&small, &large, &mx, 0, &mut vec![]);
    puts!("{}\n", ma);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
