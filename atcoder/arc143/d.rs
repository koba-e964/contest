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

fn dfs(v: usize, pre: usize, g: &[Vec<(usize, usize)>], ord: &mut [usize], cnt: &mut usize, preord: usize, ans: &mut [bool], vis: &mut [bool]) {
    if vis[v] {
        if preord > ord[v] {
            ans[pre / 2] = pre % 2 == 1;
        }
        return;
    }
    vis[v] = true;
    ord[v] = *cnt;
    if pre < 2 * ans.len() {
        ans[pre / 2] = pre % 2 == 1;
    }
    *cnt += 1;
    for &(idx, w) in &g[v] {
        if pre / 2 == idx / 2 { continue; }
        dfs(w, idx, g, ord, cnt, ord[v], ans, vis);
    }
}

// Tags: strongly-connected-components, dfs-tree
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize, m: usize,
        a: [usize1; m],
        b: [usize1; m],
    }
    let mut g = vec![vec![]; n];
    for i in 0..m {
        g[a[i]].push((2 * i, b[i]));
        g[b[i]].push((2 * i + 1, a[i]));
    }
    let mut ans = vec![false; m];
    let mut vis = vec![false; n];
    let mut ord = vec![0; n];
    let mut cnt = 0;
    for i in 0..n {
        if !vis[i] {
            dfs(i, 2 * m, &g, &mut ord, &mut cnt, 0, &mut ans, &mut vis);
        }
    }
    for i in 0..m {
        puts!("{}", if ans[i] { 1 } else { 0 });
    }
    puts!("\n");
}
