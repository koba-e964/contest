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

const INF: i64 = 1 << 50;

fn dfs(v: usize, par: usize, g: &[Vec<usize>], dp: &mut [[i64; 2]]) {
    let mut me = [-INF; 2];
    let mut s = 0;
    let mut mi = INF;
    for &w in &g[v] {
        if w == par { continue; }
        dfs(w, v, g, dp);
        let poss = max(dp[w][0], dp[w][1]);
        s += poss;
        mi = min(mi, poss - dp[w][0]);
    }
    me[0] = s;
    me[1] = s - mi + 1;
    dp[v] = me;
}

// https://yukicoder.me/problems/no/1582 (3)
// 最適値は最大マッチングの大きさ以上である。木だから最大マッチングの大きさで抑えられる。最大マッチングであるという仮定から、A 君が選んだ頂点のうちどちらかは戻しても最大マッチングが 1 減ったままになる。両方減らないと仮定するとそれで元のグラフにおいてより大きいマッチングができるため。
// Tags: max-matching-on-trees
fn solve() {
    input! {
        n: usize,
        ab: [(usize1, usize1); n - 1],
    }
    let mut g = vec![vec![]; n];
    for &(a, b) in &ab {
        g[a].push(b);
        g[b].push(a);
    }
    let mut dp = vec![[-INF; 2]; n];
    dfs(0, n, &g, &mut dp);
    println!("{}", max(dp[0][0], dp[0][1]));
}
