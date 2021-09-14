#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Read;

#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn get_word() -> String {
    let stdin = std::io::stdin();
    let mut stdin=stdin.lock();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.unwrap_or(0) == 0 || u8b[0] <= b' ' {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

/**
 * Union-Find tree.
 * Verified by https://atcoder.jp/contests/pakencamp-2019-day3/submissions/9253305
 */
struct UnionFind { disj: Vec<usize>, rank: Vec<usize> }

impl UnionFind {
    fn new(n: usize) -> Self {
        let disj = (0..n).collect();
        UnionFind { disj: disj, rank: vec![1; n] }
    }
    fn root(&mut self, x: usize) -> usize {
        if x != self.disj[x] {
            let par = self.disj[x];
            let r = self.root(par);
            self.disj[x] = r;
        }
        self.disj[x]
    }
    fn unite(&mut self, x: usize, y: usize) {
        let mut x = self.root(x);
        let mut y = self.root(y);
        if x == y { return }
        if self.rank[x] > self.rank[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.disj[x] = y;
        self.rank[y] += self.rank[x];
    }
    #[allow(unused)]
    fn is_same_set(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
    #[allow(unused)]
    fn size(&mut self, x: usize) -> usize {
        let x = self.root(x);
        self.rank[x]
    }
}

const B: usize = 21;

fn dfs1(v: usize, par: usize, g: &[Vec<(usize, usize)>],
        ch: &mut [Vec<(usize, usize)>],
        dp1: &mut [Vec<i64>]) {
    let mut me = vec![0; B];
    for &(w, c) in &g[v] {
        if w == par {
            continue;
        }
        ch[v].push((w, c));
        dfs1(w, v, g, ch, dp1);
        for i in 0..B {
            me[min(i, c)] += dp1[w][i];
        }
        me[c] += 1;
    }
    dp1[v] = me;
}

fn dfs2(v: usize, par: usize, g: &[Vec<(usize, usize)>],
        dp1: &[Vec<i64>], dp2: &mut [Vec<i64>], passed: &[i64]) {
    let mut all = passed.to_vec();
    for i in 0..B {
        all[i] += dp1[v][i];
    }
    for &(w, c) in &g[v] {
        if w == par { continue; }
        let mut nxt = all.clone();
        for i in 0..B {
            nxt[min(i, c)] -= dp1[w][i];
        }
        for j in c + 1..B {
            nxt[c] += nxt[j];
            nxt[j] = 0;
        }
        dp2[w] = nxt.clone();
        dfs2(w, v, g, dp1, dp2, &nxt);
    }
}

// Tags: rerooting
// Solved with hints
fn solve() {
    let t: usize = get();
    for case_nr in 0..t {
        let n: usize = get();
        let mut g = vec![vec![]; n];
        let mut ch = vec![vec![]; n];
        let mut e = vec![];
        for _ in 0..n - 1 {
            let a = get::<usize>() - 1;
            let b = get::<usize>() - 1;
            let c: usize = get();
            g[a].push((b, c));
            g[b].push((a, c));
            e.push((c, a, b));
        }
        e.sort(); e.reverse();
        let mut all = 0;
        let mut uf = UnionFind::new(n);
        for (c, a, b) in e {
            if uf.is_same_set(a, b) { continue; }
            let sa = uf.size(a) as i64;
            let sb = uf.size(b) as i64;
            all += sa * sb * c as i64;
            uf.unite(a, b);
        }
        let mut ans = vec![all; n];
        let mut dp1 = vec![vec![]; n];
        let mut dp2 = vec![vec![]; n];
        dfs1(0, n, &g, &mut ch, &mut dp1);
        dfs2(0, n, &g, &dp1, &mut dp2, &vec![0; B]);
        for v in 0..n {
            for &(w, c) in &ch[v] {
                dp1[w][c] += 1;
                for j in (0..B - 1).rev() {
                    dp2[w][j] += dp2[w][j + 1];
                }
                for j in (0..B - 1).rev() {
                    dp1[w][j] += dp1[w][j + 1];
                }
                let mut tmp = [0; B];
                for j in 0..c + 1 {
                    tmp[j] += dp1[w][j] * dp2[w][j];
                }
                for j in 0..c {
                    tmp[j] -= tmp[j + 1];
                }
                for j in 0..c + 1 {
                    ans[w] -= j as i64 * tmp[j];
                }
            }
        }
        let mut out = 1;
        const MOD: i64 = 1_000_000_007;
        for i in 1..n {
            out = out * (ans[i] % MOD) % MOD;
        }
        println!("Case #{}: {}", case_nr + 1, out);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 5 * 104_857_600; // 500 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
