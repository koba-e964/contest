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

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

// https://yukicoder.me/problems/no/2786 (3)
// Ref: https://betrue12.hateblo.jp/entry/2019/08/14/152227
// Similar problems: https://atcoder.jp/contests/abc394/tasks/abc394_g
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        h: usize, w: usize,
        f: [[usize; w]; h],
        q: usize,
        abcd: [(usize1, usize1, usize1, usize1); q],
    }
    const W: usize = 1 << 18;
    let mut pass = vec![W; q];
    let mut fail = vec![0; q];
    let mut con = vec![vec![]; W];
    for i in 0..h - 1 {
        for j in 0..w {
            let idx = i * w + j;
            let t = f[i][j].max(f[i + 1][j]);
            con[t].push((idx, idx + w));
        }
    }
    for i in 0..h {
        for j in 0..w - 1 {
            let idx = i * w + j;
            let t = f[i][j].max(f[i][j + 1]);
            con[t].push((idx, idx + 1));
        }
    }
    for _ in 0..18 {
        let mut uf = UnionFind::new(h * w);
        let mut ev = vec![vec![]; W];
        for i in 0..q {
            let mid = (pass[i] + fail[i]) / 2;
            ev[mid].push(i);
        }
        for i in 0..W {
            for &(x, y) in &con[i] {
                uf.unite(x, y);
            }
            for &idx in &ev[i] {
                let (a, b, c, d) = abcd[idx];
                let v0 = a * w + b;
                let v1 = c * w + d;
                if uf.is_same_set(v0, v1) {
                    pass[idx] = i;
                } else {
                    fail[idx] = i;
                }
            }
        }
    }
    for i in 0..q {
        puts!("{}\n", pass[i]);
    }
}
