use std::io::Read;

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

fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

// Quick-Find data structure.
// Verified by: https://atcoder.jp/contests/cf17-tournament-round3-open/submissions/22928265
// Verified by: https://atcoder.jp/contests/ttpc2019/submissions/23384553 (polymorphic version)
// Verified by: https://yukicoder.me/submissions/727881 (polymorphic version)
#[derive(Clone)]
struct QuickFind<T = ()> {
    root: Vec<usize>, mem: Vec<Vec<usize>>,
    dat: Vec<T>, default: T,
}

impl QuickFind<()> {
    #[allow(unused)]
    fn new(n: usize) -> Self {
        Self::with_dat(n, ())
    }
    #[allow(unused)]
    fn unite(&mut self, x: usize, y: usize) {
        self.unite_with_hooks(x, y, |&(), _| (), |(), (), _| ());
    }
}
impl<T: Clone> QuickFind<T> {
    fn with_dat(n: usize, def: T) -> Self {
        let root = (0..n).collect();
        let mut mem = vec![vec![]; n];
        for i in 0..n {
            mem[i] = vec![i];
        }
        QuickFind { root: root, mem: mem, dat: vec![def.clone(); n], default: def }
    }
    fn root(&self, x: usize) -> usize {
        self.root[x]
    }
    #[allow(unused)]
    fn set(&mut self, idx: usize, val: T) {
        self.apply(idx, move |me| *me = val);
    }
    #[allow(unused)]
    fn get(&mut self, idx: usize) -> T {
        let mut ans = self.default.clone();
        self.apply(idx, |me| ans = me.clone());
        ans
    }
    fn apply<F: FnOnce(&mut T)>(&mut self, idx: usize, f: F) {
        let r = self.root[idx];
        f(&mut self.dat[r]);
    }
    // unite always merges y to x if |x| >= |y|.
    fn unite_with_hooks<F: FnMut(&T, i64), G: FnMut(T, T, &[usize]) -> T>(
        &mut self, x: usize, y: usize,
        mut hook: F, mut merge: G) {
        let mut x = self.root(x);
        let mut y = self.root(y);
        if x == y { return }
        if self.mem[x].len() < self.mem[y].len() {
            std::mem::swap(&mut x, &mut y);
        }
        let memy = std::mem::replace(&mut self.mem[y], vec![]);
        for &v in &memy {
            self.root[v] = x;
        }
        self.mem[x].extend_from_slice(&memy);
        // hook
        hook(&self.dat[x], -1);
        hook(&self.dat[y], -1);
        self.dat[x] = merge(
            std::mem::replace(&mut self.dat[x], self.default.clone()),
            std::mem::replace(&mut self.dat[y], self.default.clone()),
            &memy,
        );
        hook(&self.dat[x], 1);
    }
    #[allow(unused)]
    fn is_same_set(&self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
    #[allow(unused)]
    fn size(&self, x: usize) -> usize {
        let x = self.root(x);
        self.mem[x].len()
    }
}

// https://yukicoder.me/problems/no/2294 (3.5)
// 30 個の桁ごとに独立な問題 (重みが 0 か 1) を解くと思うことができる。
// 連結性とある頂点からの重みの和の偶奇は UnionFind か QuickFind でできる。
// -> TLE。QuickFind そのものは複数ある必要はないが、複数持ってしまったため。
// QuickFind を 1 つにしてデータの方を 30 並列にすることで AC。
fn main() {
    let n: usize = get();
    let mut x: usize = get();
    let q: usize = get();
    let mut qf = QuickFind::with_dat(n, [(1, 0); 30]);
    let mut wei = vec![vec![0; n]; 30];
    for _ in 0..q {
        let ty: i32 = get();
        if ty == 1 {
            let v: usize = get();
            let w: u32 = get();
            qf.unite_with_hooks(x, v, |_, _| (), |a, b, less| {
                let mut res = [(0, 0); 30];
                for i in 0..30 {
                    let w = (w >> i) & 1;
                    if w != wei[i][x] ^ wei[i][v] {
                        for &v in less {
                            wei[i][v] ^= 1;
                        }
                        res[i] = (a[i].0 + b[i].1, a[i].1 + b[i].0);
                    } else {
                        res[i] = (a[i].0 + b[i].0, a[i].1 + b[i].1);
                    }
                }
                res
            });
            continue;
        }
        if ty == 2 {
            let u: usize = get();
            let v: usize = get();
            if qf.root(u) != qf.root(v) {
                println!("-1");
                continue;
            }
            let mut ans = 0u32;
            for i in 0..30 {
                ans |= (wei[i][u] ^ wei[i][v]) << i;
            }
            println!("{}", ans);
            x = (x + ans as usize) % n;
            continue;
        }
        if ty == 3 {
            let v: usize = get();
            let mut ans = 0i64;
            const MOD: i64 = 998_244_353;
            let val = qf.get(v);
            for i in 0..30 {
                let (a, b) = val[i];
                ans += (a as i64 * b as i64) % MOD << i;
                ans %= MOD;
            }
            println!("{}", ans);
            continue;
        }
        let val: usize = get();
        x = (x + val) % n;
    }
}
