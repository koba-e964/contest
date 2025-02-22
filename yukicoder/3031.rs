use std::collections::*;
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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn dfs(v: usize, g: &[Vec<(usize, bool)>], vis: &mut [bool], pot: &mut [bool], p: bool) -> Option<()>{
    if vis[v] {
        if pot[v] != p {
            return None;
        }
        return Some(());
    }
    vis[v] = true;
    pot[v] = p;
    for &(u, sw) in &g[v] {
        dfs(u, g, vis, pot, p ^ sw)?;
    }
    Some(())
}

// https://yukicoder.me/problems/no/3031 (3)
// 三角形によって与えられた曲面の向き付けが可能かどうかを判定する問題。
// 三角形の辺が 3 個以上の三角形に共有されていたら NO である。なので三角形の辺は高々 2 個の三角形に共有されているとしてよい。
// 三角形を頂点、三角形の辺の共有を辺としたグラフを考える。各頂点の次数は高々 3 である。DFS をすればよい。
fn solve() {
    input! {
        m: usize,
        abc: [[i32; 3]; m],
    }
    let mut seen = HashMap::new();
    for i in 0..m {
        for j in 0..3 {
            let x = abc[i][j];
            let y = abc[i][(j + 1) % 3];
            let (x, y, sw) = if x < y {
                (x, y, false)
            } else {
                (y, x, true)
            };
            seen.entry((x, y)).or_insert(vec![]).push((i, sw));
            if seen[&(x, y)].len() >= 3 {
                println!("NO");
                return;
            }
        }
    }
    let mut g = vec![vec![]; m];
    for (_, v) in seen {
        if v.len() != 2 {
            continue;
        }
        let (a, sw0) = v[0];
        let (c, sw1) = v[1];
        g[a].push((c, sw0 == sw1));
        g[c].push((a, sw0 == sw1));
    }
    let mut vis = vec![false; m];
    let mut pot = vec![false; m];
    for i in 0..m {
        if !vis[i] {
            if dfs(i, &g, &mut vis, &mut pot, false).is_none() {
                println!("NO");
                return;
            }
        }
    }
    println!("YES");
}
