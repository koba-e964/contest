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

fn dfs1(v: usize, par: usize, g: &[Vec<(usize, u32)>], dp: &mut [u32], x: u32) {
    dp[v] = x;
    for &(w, c) in &g[v] {
        if par == w { continue; }
        dfs1(w, v, g, dp, x ^ c);
    }
}

fn dfs2(v: usize, par: usize, g: &[Vec<(usize, u32)>], coo: &[u32],
        dp: &[u32], pre: &mut [usize],
        ch: &mut [Vec<usize>], root: &mut [Vec<usize>]) {
    let idx = coo.binary_search(&dp[v]).unwrap();
    let n = g.len();
    if pre[idx] == n {
        root[idx].push(v);
    } else {
        ch[pre[idx]].push(v);
    }
    let oldpre = pre[idx];
    for &(w, _) in &g[v] {
        if par == w { continue; }
        pre[idx] = w;
        dfs2(w, v, g, coo, dp, pre, ch, root);
    }
    pre[idx] = oldpre;
}

fn dfs3(v: usize, par: usize, g: &[Vec<(usize, u32)>], sz: &mut [i32]) {
    let mut s = 1;
    for &(w, _) in &g[v] {
        if par == w { continue; }
        dfs3(w, v, g, sz);
        s += sz[w];
    }
    sz[v] = s;
}

// https://yukicoder.me/problems/no/1769 (4)
// 根を適当に決め根からの重みの xor をとることで、以下の問題と同値。
// 「頂点 v に値 a[v] が振られている。ペア (i, j) であって、i から j へのパスの中に a[i] と同じ値が (頂点 i を除いて) 存在しないものの個数を求めよ。」
// a[v] を座標圧縮して、v に対して a[i] = a[v] かつ i は v の先祖であるような一番低い i が計算できるように dfs する。そうするとある値に挟まれた連結成分というものがわかる。
// 連結成分の大きさ (端点を除く) を s、連結成分の端点の個数を t とすると、その連結成分の端点が始点であるようなペアは st 個。
// -> 上で計算した (v, i) ペアについて、i の異なる子の子孫であるような v は別の連結成分に属するので区別しないといけない。dfs の際に a[v] ごとに w を覚えておけば良い。また root を含む (<=> 先祖の中に同じ a の値をもつ頂点が存在しない) 連結成分は a の値ごとに別に数える必要がある。
// Tags: subtree, colored-tree
fn solve() {
    input! {
        n: usize,
        abc: [(usize1, usize1, u32); n - 1],
    }
    let mut g = vec![vec![]; n];
    for &(a, b, c) in &abc {
        g[a].push((b, c));
        g[b].push((a, c));
    }
    let mut dp = vec![0; n];
    dfs1(0, n, &g, &mut dp, 0);
    let mut coo = dp.clone();
    coo.sort(); coo.dedup();
    let m = coo.len();
    let mut pre = vec![n; m];
    let mut ch = vec![vec![]; n];
    let mut root = vec![vec![]; m];
    dfs2(0, n, &g, &coo, &dp, &mut pre, &mut ch, &mut root);
    let mut sz = vec![0; n];
    dfs3(0, n, &g, &mut sz);
    let mut ans = 0i64;
    for i in 0..m {
        let mut cs = n as i32;
        let mut t = 0;
        for &v in &root[i] {
            cs -= sz[v];
            t += 1;
        }
        ans += cs as i64 * t;
    }
    for i in 1..n {
        let mut cs = sz[i];
        let mut t = 1;
        for &v in &ch[i] {
            cs -= sz[v];
            t += 1;
        }
        ans += cs as i64 * t;
    }
    println!("{}", ans);
}
