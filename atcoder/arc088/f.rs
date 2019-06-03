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

const INF: usize = 1 << 30;

fn fold(pool: &[usize], x: usize) -> usize {
    assert_eq!(pool.len() % 2, 0);
    let n = pool.len();
    for i in 0..n / 2 {
        if pool[n - i - 1] + pool[i] > x {
            return INF;
        }
    }
    0
}

fn opt(pool: &[usize], x: usize) -> usize {
    let n = pool.len();
    assert_eq!(n % 2, 1);
    let mut pass = n as i32;
    let mut fail = -1;
    while pass - fail > 1 {
        let mid = (pass + fail) / 2;
        let idx = mid as usize;
        let mut ok = true;
        for i in 0..n / 2 {
            let a = if idx <= i { i + 1 } else { i };
            let b = if idx <= n - i - 2 { n - i - 1 } else { n - i - 2 };
            if pool[a] + pool[b] > x {
                ok = false;
                break;
            }
        }
        if ok {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    if pass < n as i32 {
        pool[pass as usize]
    } else {
        INF
    }
}

fn dfs(v: usize, par: usize, g: &[Vec<usize>], x: usize) -> usize {
    let n = g.len();
    let mut pool = vec![];
    if n == par {
        // Only one child
        let ch = g[v][0];
        let dep = dfs(ch, v, g, x);
        return dep + 1;
    }
    for &w in &g[v] {
        if w == par { continue; }
        let sub = dfs(w, v, g, x);
        pool.push(sub + 1);
    }
    if pool.len() == 0 {
        return 0;
    }
    pool.sort();
    let mut mi = INF;
    if pool.len() % 2 == 0 {
        if pool[pool.len() - 1] > x {
            return x + 1;
        }
        mi = opt(&pool[..pool.len() - 1], x);
        mi = min(mi, fold(&pool, x));
    } else {
        mi = min(mi, opt(&pool, x));
    }
    mi
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        uv: [(usize1, usize1); n - 1],
    }
    let mut g = vec![vec![]; n];
    for &(a, b) in &uv {
        g[a].push(b);
        g[b].push(a);
    }
    let mut a = 1;
    for i in 0..n {
        let d = g[i].len();
        if d >= 3 {
            a += (d - 1) / 2;
        }
    }
    let mut r = 0;
    for i in 0..n {
        if g[i].len() == 1 {
            r = i;
            break;
        }
    }
    // Find the smallest b by binsect.
    let mut pass = n - 1;
    let mut fail = 0;
    while pass - fail > 1 {
        let mid = (pass + fail) / 2;
        let res = dfs(r, n, &g, mid);
        if res <= mid {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    puts!("{} {}\n", a, pass);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
