use std::cmp::*;
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

const MOD: i64 = 1_000_000_007;

fn dfs(
    v: usize, g: &[Vec<(usize, i64)>], k: i64,
    vis: &mut [bool], eq: &mut Vec<i64>, pot: &mut [(i64, i64)],
    p: (i64, i64),
) -> (i64, i64) {
    let (mut lo, mut hi) = if p.0 == 1 {
        (1 - p.1, k - p.1)
    } else {
        (p.1 - k, p.1 - 1)
    };
    if vis[v] {
        if pot[v].0 == p.0 {
            return if pot[v].1 == p.1 {
                (lo, hi)
            } else {
                (1, 0) // contradiction
            };
        }
        eq.push((p.1 - pot[v].1) * 2 / (pot[v].0 - p.0));
        return (lo, hi);
    }
    vis[v] = true;
    pot[v] = p;
    for &(w, c) in &g[v] {
        let (sublo, subhi) = dfs(w, g, k, vis, eq, pot, (-p.0, c - p.1));
        lo = max(lo, sublo);
        hi = min(hi, subhi);
    }
    (lo, hi)
}

fn calc(n: usize, xyz: &[(usize, usize, i64)], k: i64) -> i64 {
    let mut g = vec![vec![]; n];
    for &(x, y, z) in xyz {
        g[x].push((y, z));
        g[y].push((x, z));
    }
    let mut vis = vec![false; n];
    let mut ans = 1;
    let mut pot = vec![(0, 0); n];
    for i in 0..n {
        let mut eq = vec![];
        if !vis[i] {
            let (mi, ma) = dfs(i, &g, k, &mut vis, &mut eq, &mut pot, (1, 0));
            eq.dedup();
            if mi > ma || eq.len() >= 2 {
                return 0;
            }
            if eq.len() == 1 {
                let x = eq[0];
                if x % 2 != 0 || mi > x / 2 || x / 2 > ma {
                    return 0;
                }
            } else {
                ans = ans * (ma - mi + 1) % MOD;
            }
        }
    }
    ans
}

// https://yukicoder.me/problems/no/1502 (3.5)
// <= k に変換すれば DFS で制約を見つけるだけになる。
fn solve() {
    input! {
        n: usize, m: usize, k: i64,
        xyz: [(usize1, usize1, i64); m],
    }
    println!("{}", (calc(n, &xyz, k) - calc(n, &xyz, k - 1) + MOD) % MOD);
}
