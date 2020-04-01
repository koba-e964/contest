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

/// Rerooting.
/// Verified by: ABC160-F
/// (https://atcoder.jp/contests/abc160/submissions/11378396)
/// f: merge
/// d: deepen (adds a node to a collection of child nodes)
struct Reroot<T> {
    g: Vec<Vec<usize>>,
    zero: T,
    dp1: Vec<T>,
    dp2: Vec<T>,
    dp: Vec<T>,
    ch: Vec<Vec<usize>>,
    acc_l: Vec<Vec<T>>,
    acc_r: Vec<Vec<T>>,
}

impl<T: Copy> Reroot<T> {
    pub fn new(g: &[Vec<usize>], zero: T) -> Reroot<T> {
        let n = g.len();
        Reroot {
            g: g.to_vec(),
            zero: zero.clone(),
            dp1: vec![zero.clone(); n],
            dp2: vec![zero.clone(); n],
            dp: vec![zero.clone(); n],
            ch: vec![vec![]; n],
            acc_l: vec![vec![]; n],
            acc_r: vec![vec![]; n],
        }
    }
    // TODO include f in struct
    pub fn do_comp<F: FnMut(T, T) -> T, D: FnMut(T, usize) -> T>(
        &mut self,
        f: &mut F,
        d: &mut D,
    ) {
        let n = self.g.len();
        Self::dfs1(0, n, &self.g, &mut self.dp1, &mut self.ch,
                   &mut self.acc_l, &mut self.acc_r, self.zero, f, d);
        Self::dfs2(0, &self.ch, &self.dp1, &mut self.dp2,
                   self.zero,
                   &self.acc_l, &self.acc_r, self.zero, f, d);
        self.dp[0] = self.dp1[0];
        for i in 1..n {
            self.dp[i] = d(f(self.acc_r[i][0], self.dp2[i]), i);
        }
    }
    fn dfs1<F: FnMut(T, T) -> T, D: FnMut(T, usize) -> T>(
        v: usize, par: usize, g: &[Vec<usize>],
        dp1: &mut [T],
        ch: &mut [Vec<usize>],
        acc_l: &mut [Vec<T>], acc_r: &mut [Vec<T>],
        zero: T,
        f: &mut F,
        d: &mut D,
    ) {
        let mut ary = vec![];
        let mut mych = vec![];
        for &w in &g[v] {
            if w == par { continue; }
            mych.push(w);
            Self::dfs1(w, v, g, dp1, ch, acc_l, acc_r, zero.clone(), f, d);
            ary.push(dp1[w].clone());
        }
        let m = ary.len();
        acc_l[v] = vec![zero.clone(); m + 1];
        acc_r[v] = vec![zero.clone(); m + 1];
        for i in 0..m {
            let val = f(acc_l[v][i], ary[i]);
            acc_l[v][i + 1] = val;
        }
        for i in (0..m).rev() {
            let val = f(acc_r[v][i + 1], ary[i]);
            acc_r[v][i] = val;
        }
        ch[v] = mych;
        dp1[v] = d(acc_r[v][0], v);
    }
    fn dfs2<F: FnMut(T, T) -> T, D: FnMut(T, usize) -> T>(
        v: usize, ch: &[Vec<usize>],
        dp1: &[T],
        dp2: &mut [T],
        passed: T,
        acc_l: &[Vec<T>], acc_r: &[Vec<T>],
        zero: T,
        f: &mut F,
        d: &mut D,
    ) {
        dp2[v] = passed;
        for i in 0..ch[v].len() {
            let w = ch[v][i];
            let leave_one = f(acc_l[v][i], acc_r[v][i + 1]);
            Self::dfs2(w, ch, dp1, dp2, d(f(leave_one, passed), v),
                       acc_l, acc_r,
                       zero, f, d);
        }
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
        a: [i32; n],
        uv: [(usize1, usize1); n - 1],
    }
    let mut g = vec![vec![]; n];
    for &(u, v) in &uv {
        g[u].push(v);
        g[v].push(u);
    }
    let diff: Vec<i64> =
        a.iter().map(|&v| if v == 0 { -1 } else { 1 }).collect();
    let mut reroot = Reroot::new(&g, 0);
    let mut f = |x, y| max(0, x) + max(0, y);
    let mut d = |x, v| x + diff[v];
    reroot.do_comp(&mut f, &mut d);
    for i in 0..n {
        puts!("{}{}", reroot.dp[i],
              if i + 1 == n { "\n" } else {" " });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
