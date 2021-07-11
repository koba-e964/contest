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

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn dfs(rem: &mut Vec<usize>, path: &mut Vec<usize>,
       found: &mut Vec<Vec<usize>>, k: usize,
       g: &[Vec<usize>], indeg: &mut [usize]) {
    let n = g.len();
    if found.len() >= k {
        return;
    }
    if path.len() >= n {
        found.push(path.clone());
        return;
    }
    if rem.is_empty() {
        return;
    }
    let pos = rem.len() - 1;
    for i in 0..min(k, rem.len()) {
        if found.len() >= k {
            break;
        }
        rem.swap(pos - i, pos);
        let x = rem.pop().unwrap();
        for &w in &g[x] {
            indeg[w] -= 1;
            if indeg[w] == 0 {
                rem.push(w);
            }
        }
        path.push(x);
        dfs(rem, path, found, k, g, indeg);
        path.pop();
        for &w in &g[x] {
            indeg[w] += 1;
        }
        rem.truncate(pos);
        rem.push(x);
        rem.swap(pos - i, pos);
    }
}

// Tags: dfs, topological-sort
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    #[allow(unused)]
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize, m: usize, k: usize,
        ab: [(usize1, usize1); m],
    }
    let mut g = vec![vec![]; n];
    let mut indeg = vec![0; n];
    for &(a, b) in &ab {
        g[a].push(b);
        indeg[b] += 1;
    }
    let mut st = vec![];
    for i in 0..n {
        if indeg[i] == 0 {
            st.push(i);
        }
    }
    let mut ans = vec![];
    dfs(&mut st, &mut vec![], &mut ans, 1, &g, &mut indeg);
    if ans.is_empty() {
        puts!("-1\n");
        return;
    }
    ans.clear();
    dfs(&mut st, &mut vec![], &mut ans, k, &g, &mut indeg);
    if ans.len() < k {
        puts!("-1\n");
    } else {
        for i in 0..k {
            for v in &mut ans[i] { *v += 1; }
            putvec!(ans[i]);
        }
    }
}
