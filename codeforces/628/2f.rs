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

// Ok: indep set, Err: cycle
fn dfs(v: usize, g: &[Vec<usize>], vis: &mut [bool], picked: &mut [bool],
       dep: &mut [i32], d: i32, sq: i32) -> Result<(), (usize, Vec<usize>)> {
    let n = g.len();
    dep[v] = d;
    vis[v] = true;
    let mut to_picked = false;
    for &w in &g[v] {
        if vis[w] {
            if dep[w] <= dep[v] - sq + 1 {
                return Err((w, vec![v]));
            }
        } else if let Err((end, mut cyc)) = dfs(w, g, vis, picked, dep, d + 1, sq) {
            if end >= n {
                return Err((n, cyc));
            }
            cyc.push(v);
            return Err((if v == end { n } else { end }, cyc));
        }
        to_picked |= picked[w];
    }
    picked[v] = !to_picked;
    Ok(())
}


// Editorial solution 1
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize,
        ab: [(usize1, usize1)],
    }
    let mut g = vec![vec![]; n];
    for &(a, b) in &ab {
        g[a].push(b);
        g[b].push(a);
    }
    let mut sq = 0;
    while sq * sq < n {
        sq += 1;
    }
    let mut vis = vec![false; n];
    let mut picked = vec![false; n];
    let mut dep = vec![0; n];
    let res = dfs(0, &g, &mut vis, &mut picked, &mut dep, 0, sq as i32);
    if let Err((_, cyc)) = res {
        puts!("2\n{}\n", cyc.len());
        for i in 0..cyc.len() {
            puts!("{}{}", cyc[i] + 1, if i + 1 == cyc.len() { "\n" } else { " " });
        }
    } else {
        puts!("1\n");
        let mut cnt = 0;
        for i in 0..n {
            if picked[i] {
                cnt += 1;
                puts!("{}{}", i + 1, if cnt == sq { "\n" } else { " " });
            }
            if cnt >= sq { break; }
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
