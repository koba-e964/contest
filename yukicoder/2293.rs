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

// https://yukicoder.me/problems/no/2293 (3)
// クリアなしであれば UnionFind でできる。クリアする時、UnionFind を初期状態に戻す必要があるが、
// そのとき変更すべき場所は今まで UnionFind に unite の引数として与えた頂点だけである。
// よって、クエリ 1 個につき高々 1 回そのようなクリアを行う必要があるため、合計　O(Q alpha(N))-time でできる。
// Tags: operations-per-something, finding-upper-bounds-of-complexity, reusing-data-structures
fn main() {
    let n: usize = get();
    let q: usize = get();
    let mut pw2 = vec![0; n + 1];
    pw2[0] = 1;
    const MOD: i64 = 998_244_353;
    for i in 1..n + 1 {
        pw2[i] = pw2[i - 1] * 2 % MOD;
    }
    let mut uf = UnionFind::new(2 * n);
    let mut conn = n;
    let mut contra = false;
    let mut seen = vec![];
    for _ in 0..q {
        let ty: i32 = get();
        if ty == 3 {
            for &v in &seen {
                uf.disj[v] = v;
                uf.rank[v] = 1;
            }
            conn = n;
            contra = false;
            seen.clear();
            println!("{}", pw2[n]);
            continue;
        }
        let u = get::<usize>() - 1;
        let v = get::<usize>() - 1;
        if !contra {
            let x = (ty - 1) as usize;
            if !uf.is_same_set(2 * u, 2 * v + x) {
                uf.unite(2 * u, 2 * v + x);
                uf.unite(2 * u + 1, 2 * v + 1 - x);
                seen.push(2 * u);
                seen.push(2 * u + 1);
                seen.push(2 * v);
                seen.push(2 * v + 1);
                conn -= 1;
            }
            if uf.is_same_set(2 * u, 2 * u + 1) {
                contra = true;
            }
        }
        println!("{}", if contra { 0 } else { pw2[conn] });
    }
}
