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

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn dfs(v: usize, ch: &[Vec<usize>], filled: &[bool], depths: &mut Vec<(usize, usize)>, d: usize) {
    if filled[v] { return; }
    depths.push((d, v));
    for &w in ch[v].iter() {
        dfs(w, ch, filled, depths, d + 1);
    }
}

fn get_depth(root: usize, dep: &mut [usize], ch: &[Vec<usize>], d: usize) {
    dep[root] = d;
    for &w in ch[root].iter() {
        get_depth(w, dep, ch, d + 1);
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    input! {
        n: usize,
        m: usize,
        k: usize,
        p: [usize; n],
    }
    let mut ch = vec![vec![]; n];
    let mut root = 0;
    for i in 0..n {
        if p[i] == 0 {
            root = i;
        } else {
            ch[p[i] - 1].push(i);
        }
    }
    let mut remaining = k;
    let mut filled = vec![false; n];
    let mut ans = vec![n; m];
    let mut dep = vec![0; n];
    get_depth(root, &mut dep, &ch, 1);
    for i in 0..m {
        let mut nxt = None;
        let mut reachable = vec![];
        dfs(root, &ch, &filled, &mut reachable, 1);
        let mut reachability = vec![false; n];
        for (_, v) in reachable {
            reachability[v] = true;
        }
        for j in 0..n {
            if !reachability[j] { continue; }
            if nxt.is_some() { break; }
            if filled[j] { continue; }
            let mut depths = vec![];
            filled[j] = true;
            dfs(root, &ch, &filled, &mut depths, 1);
            filled[j] = false;
            if depths.len() < m - i - 1 {
                continue;
            }
            depths.sort();
            let mut lo = dep[j];
            let mut hi = dep[j];
            for l in 0..m - i - 1 {
                lo += depths[l].0;
                hi += depths[depths.len() - l - 1].0;
            }
            if lo <= remaining && remaining <= hi {
                nxt = Some(j);
            }
        }
        let nxt = match nxt {
            None => {
                assert_eq!(i, 0);
                puts!("-1\n");
                return;
            },
            Some(v) => v,
        };
        ans[i] = nxt;
        remaining -= dep[nxt];
        filled[nxt] = true;
    }
    for i in 0..m {
        puts!("{}{}", ans[i] + 1, if i == m - 1 { "\n" } else { " " });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
