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

fn dfs(v: usize, par: usize, g: &[Vec<usize>], dp: &mut [usize], t: usize) {
    dp[v] = t;
    for &w in &g[v] {
        if w == par {
            continue;
        }
        dfs(w, v, g, dp, 3 - t);
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize,
        ab: [(usize1, usize1); n - 1],
    }
    let mut dp = vec![0; n];
    let mut g = vec![vec![]; n];
    for &(a, b) in &ab {
        g[a].push(b);
        g[b].push(a);
    }
    dfs(0, n, &g, &mut dp, 1);
    let mut col = vec![vec![]; 3];
    for i in 0..n {
        col[dp[i]].push(i);
    }
    let mut ans = vec![0; n];
    let mi = min(col[1].len(), col[2].len());
    let mut rem: BTreeSet<usize> = (1..n + 1).collect();
    if mi <= n / 3 {
        let idx = if col[1].len() < col[2].len() { 1 } else { 2 };
        let mut cnt = 1;
        for &v in &col[idx] {
            ans[v] = 3 * cnt;
            rem.remove(&(3 * cnt));
            cnt += 1;
        }
        let mut it = rem.iter();
        for i in 0..n {
            if ans[i] == 0 {
                ans[i] = *it.next().unwrap();
            }
        }
    } else {
        let idx = if col[1].len() < col[2].len() { 1 } else { 2 };
        // idx holds 2 mod 3, 3 - idx holds 1 mod 3
        for i in 0..min(col[idx].len(), (n - 2) / 3 + 1) {
            ans[col[idx][i]] = 3 * i + 2;
            rem.remove(&(3 * i + 2));
        }
        for i in 0..min(col[3 - idx].len(), (n - 1) / 3 + 1) {
            ans[col[3 - idx][i]] = 3 * i + 1;
            rem.remove(&(3 * i + 1));
        }
        let mut it = rem.iter();
        for i in 0..n {
            if ans[i] == 0 {
                ans[i] = *it.next().unwrap();
            }
        }
    }
    for i in 0..n {
        puts!("{}{}", ans[i], if i + 1 == n { "\n" } else { " " });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
