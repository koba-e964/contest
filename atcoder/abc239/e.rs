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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

const K: usize = 20;

fn dfs(v: usize, par: usize, g: &[Vec<usize>], x: &[i64],
       qs: &[Vec<(usize, usize)>],
       ans: &mut [i64]) -> Vec<i64> {
    let mut a = vec![x[v]];
    for &w in &g[v] {
        if w == par { continue; }
        let sub = dfs(w, v, g, x, qs, ans);
        a.extend(&sub);
    }
    a.sort(); a.reverse();
    a.truncate(K);
    for &(idx, pos) in &qs[v] {
        ans[idx] = a[pos];
    }
    a
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, q: usize,
        x: [i64; n],
        ab: [(usize1, usize1); n - 1],
        vk: [(usize1, usize1); q],
    }
    let mut g = vec![vec![]; n];
    for (a, b) in ab {
        g[a].push(b);
        g[b].push(a);
    }
    let mut qs = vec![vec![]; n];
    for i in 0..q {
        let (v, k) = vk[i];
        qs[v].push((i, k));
    }
    let mut ans = vec![-1; q];
    dfs(0, n, &g, &x, &qs, &mut ans);
    for a in ans {
        puts!("{}\n", a);
    }
}
