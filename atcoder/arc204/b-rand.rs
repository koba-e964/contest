
use std::collections::*;

mod sol {
    include!("b.rs");
}

// Union-Find tree.
// Verified by https://atcoder.jp/contests/pakencamp-2019-day3/submissions/9253305
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

struct Rng {
    x: u64,
}

impl Rng {
    fn new() -> Self {
        use std::hash::{Hasher, BuildHasher};
        let hm = std::collections::HashMap::<i32, i32>::new();
        let mut hash = hm.hasher().build_hasher();
        hash.write_u32(8128);
        Rng {
            x: hash.finish(),
        }
    }
    fn next(&mut self) -> u32 {
        let a = 0xdead_c0de_0013_3331u64;
        let b = 2457;
        self.x = self.x.wrapping_mul(a).wrapping_add(b);
        let x = self.x;
        ((x ^ x << 10) >> 32) as _
    }
}

fn dfs(a: &mut [usize], n: usize, k: usize, hm: &mut HashMap<Vec<usize>, (i32, i32)>) -> (i32, i32) {
    assert_eq!(a.len(), n * k, "a = {a:?}, n = {n}, k = {k}");
    if (0..n * k).all(|i| a[i] == i) {
        return (0, 0);
    }
    if let Some(&v) = hm.get(a) {
        return v;
    }
    let mut uf = UnionFind::new(n * k);
    for i in 0..n * k {
        uf.unite(a[i], i);
    }
    let mut ans = (1 << 28, 0);
    for i in 0..n * k {
        for j in i + 1..n * k {
            if !uf.is_same_set(i, j) { continue; }
            a.swap(i, j);
            let mut sub = dfs(a, n, k, hm);
            sub.0 += 1;
            if (j - i) % n == 0 {
                sub.1 -= 1;
            }
            a.swap(i, j);
            ans = std::cmp::min(ans, sub);
        }
    }
    hm.insert(a.to_vec(), ans);
    ans
}

fn naive(mut a: Vec<usize>, n: usize, k: usize) -> usize {
    let mut hm = HashMap::new();
    (-dfs(&mut a, n, k, &mut hm).1) as _
}

fn main() {
    let n = 4;
    let k = 2;
    let mut rng = Rng::new();
    for _ in 0..100 {
        let mut p = vec![0; n * k];
        for i in 0..n * k {
            p[i] = i;
        }
        for i in 1..n * k {
            let r = rng.next() as usize % (i + 1);
            p.swap(r, i);
        }
        let naive = naive(p.clone(), n, k);
        let sol = sol::calc(p.clone(), n, k);
        if naive != sol {
            eprintln!("p = {p:?}, n = {n}, k = {k}, naive = {naive}, sol = {sol}");
            panic!();
        }
    }
}
