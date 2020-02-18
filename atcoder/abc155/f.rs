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

trait Bisect<T> {
    fn lower_bound(&self, val: &T) -> usize;
    fn upper_bound(&self, val: &T) -> usize;
}

impl<T: Ord> Bisect<T> for [T] {
    fn lower_bound(&self, val: &T) -> usize {
        let mut pass = self.len() + 1;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if &self[mid - 1] >= val {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        pass - 1
    }
    fn upper_bound(&self, val: &T) -> usize {
        let mut pass = self.len() + 1;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if &self[mid - 1] > val {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        pass - 1
    }
}

fn dfs(v: usize, g: &[Vec<(usize, usize)>], vis: &mut [bool],
       ans: &mut Vec<usize>,
       tbl: &mut [i32],
) -> i32 {
    if vis[v] {
        return 0;
    }
    vis[v] = true;
    let mut cur = tbl[v];
    for &(w, idx) in &g[v] {
        let res = dfs(w, g, vis, ans, tbl);
        if res == 1 {
            cur ^= 1;
            ans.push(idx);
        }
    }
    tbl[v] = 0;
    cur
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize, m: usize,
        ab: [(i64, i32); n],
        lr: [(i64, i64); m],
    }
    let mut abi: Vec<_> = ab
        .into_iter().enumerate().map(|(i, (a, b))| (a, b, i))
        .collect();
    abi.sort();
    let mut trans = vec![(0, 0, 0); m];
    for i in 0..m {
        let (l, r) = lr[i];
        let x = abi.lower_bound(&(l, 0, 0));
        let y = abi.upper_bound(&(r, 2, 0));
        trans[i] = (x, y, i);
    }
    let mut g = vec![vec![]; n + 1];
    for &(l, r, idx) in &trans {
        g[l].push((r, idx));
        g[r].push((l, idx));
    }
    let mut ans = vec![];
    let mut vis = vec![false; n + 1];
    let mut tbl = vec![0; n + 1];
    for i in 0..n {
        tbl[i] = abi[i].1;
    }
    for i in (1..n + 1).rev() {
        tbl[i] ^= tbl[i - 1];
    }
    for i in 0..n {
        if !vis[i] {
            if dfs(i, &g, &mut vis, &mut ans, &mut tbl) != 0 {
                puts!("-1\n");
                return;
            }
        }
    }
    ans.sort();
    puts!("{}\n", ans.len());
    for i in 0..ans.len() {
        puts!("{}{}", ans[i] + 1, if i + 1 == ans.len() { "\n" } else { " " });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
