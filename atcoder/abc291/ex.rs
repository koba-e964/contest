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
            bytes.by_ref().map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr,) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// Returns (root, children)
// This functions uses O(n) stack space.
// Complexity: O(n log n)-time, O(n)-space
fn centroid_decompose(g: &[Vec<usize>]) -> (usize, Vec<Vec<usize>>) {
    fn find_subtree_sizes(g: &[Vec<usize>], v: usize, par: usize,
                          dp: &mut [usize], vis: &[bool]) {
        let mut sum = 1;
        for &w in &g[v] {
            if par == w || vis[w] { continue; }
            find_subtree_sizes(g, w, v, dp, vis);
            sum += dp[w];
        }
        dp[v] = sum;
    }
    fn centroid_decompose_inner(g: &[Vec<usize>], v: usize, par: usize,
                                ch: &mut [Vec<usize>],
                                dp: &mut [usize], vis: &mut [bool]) -> usize {
        let n = g.len();
        find_subtree_sizes(g, v, n, dp, vis);
        let cent = {
            let sz = dp[v];
            let find_centroid = |mut v: usize, mut par: usize| {
                loop {
                    let mut has_majority = false;
                    for &w in &g[v] {
                        if par == w || vis[w] { continue; }
                        if dp[w] > sz / 2 {
                            par = v;
                            v = w;
                            has_majority = true;
                            break;
                        }
                    }
                    if !has_majority {
                        return v;
                    }
                }
            };
            find_centroid(v, n)
        };
        if par < n {
            ch[par].push(cent);
        }
        // v was selected as a centroid
        // and will be ignored in the following decomposition procedure
        vis[cent] = true;
        for &w in &g[cent] {
            if !vis[w] {
                centroid_decompose_inner(g, w, cent, ch, dp, vis);
            }
        }
        cent
    }
    let n = g.len();
    let mut ch = vec![vec![]; n];
    // This Vec is used across multiple calls to `centroid_decompose_inner`
    let mut dp = vec![0; n];
    let mut vis = vec![false; n];
    let root = centroid_decompose_inner(&g, 0, n, &mut ch, &mut dp, &mut vis);
    (root, ch)
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize,
        ab: [(usize1, usize1); n - 1],
    }
    let mut g = vec![vec![]; n];
    for (a, b) in ab {
        g[a].push(b);
        g[b].push(a);
    }
    let (_root, ch) = centroid_decompose(&g);
    let mut ans = vec![-1; n];
    for i in 0..n {
        for &v in &ch[i] {
            ans[v] = i as i32 + 1;
        }
    }
    putvec!(ans);
}
