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

// Finds a cycle. Not necessarily an induced cycle.
fn dfs1(v: usize, g: &[Vec<usize>], vis: &mut [bool], memo: &mut [bool])
        -> Result<(), Vec<usize>> {
    if vis[v] {
        return if memo[v] { Err(vec![v]) } else { Ok(()) };
    }
    vis[v] = true;
    memo[v] = true;
    for &w in &g[v] {
        if let Err(mut path) = dfs1(w, g, vis, memo) {
            if path.len() <= 1 || path[0] != path[path.len() - 1] {
                path.push(v);
            }
            return Err(path);
        }
    }
    memo[v] = false;
    Ok(())
}

fn restrict(mut g: Vec<Vec<usize>>, set: &HashSet<usize>) -> Vec<Vec<usize>> {
    let n = g.len();
    for i in 0..n {
        let v = std::mem::replace(&mut g[i], vec![]);
        if !set.contains(&i) {
            continue;
        }
        g[i] = v.into_iter().filter(|v| set.contains(v)).collect();
    }
    g
}

// Contract a loop and returns a shorter loop, if any exists.
fn shorten(sub: &Vec<usize>, g: &[Vec<usize>]) -> Vec<usize> {
    let n = g.len();
    let mut seen = vec![false; n];
    let m = sub.len();
    for i in 0..m {
        let v = sub[i];
        let nxt = sub[(i + 1) % m];
        for &w in &g[v] {
            if w == nxt { continue; }
            if seen[w] {
                // w -> ... -> v (-> w)
                let pos = sub.iter().position(|&v| v == w).unwrap();
                return sub[pos..i + 1].to_vec();
            }
            // (v ->) w -> ... -> sub[0] -> v
            let pos = sub.iter().position(|&v| v == w).unwrap();
            let mut ret = sub[pos..].to_vec();
            ret.extend_from_slice(&sub[..i + 1]);
            return ret;
        }
        seen[v] = true;
    }
    vec![]
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize, m: usize,
        ab: [(usize1, usize1); m],
    }
    let mut g = vec![vec![]; n];
    let mut mat = vec![vec![false; n]; n];
    let mut vis = vec![false; n];
    for &(a, b) in &ab {
        g[a].push(b);
        mat[a][b] = true;
    }
    let mut memo = vec![false; n];
    let mut sub = Ok(());
    for i in 0..n {
        let sub_tmp = dfs1(i, &g, &mut vis, &mut memo);
        if sub_tmp.is_err() {
            sub = sub_tmp;
            break;
        }
    }
    let mut sub = match sub {
        Ok(()) => {
            puts!("-1\n");
            return;
        }
        Err(x) => x,
    };
    sub.pop().unwrap();
    sub.reverse();
    let set = sub.iter().cloned().collect();
    let mut g = restrict(g, &set);
    loop {
        // debugln!("sub = {:?}, g = {:?}", sub, g);
        let k = shorten(&sub, &g);
        let set = k.iter().cloned().collect();
        g = restrict(g, &set);
        if k.len() == 0 {
            break;
        }
        sub = k;
    }
    puts!("{}\n", sub.len());
    for v in sub {
        puts!("{}\n", v + 1);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
