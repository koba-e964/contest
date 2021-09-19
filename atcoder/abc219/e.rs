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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

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

fn ok(bits: usize) -> bool {
    let mut uf = UnionFind::new(36);
    let mut tbl = vec![vec![0; 6]; 6];
    for i in 0..4 {
        for j in 0..4 {
            tbl[1 + i][1 + j] = if (bits & 1 << (4 * i + j)) != 0 {
                1
            } else {
                0
            };
        }
    }
    for i in 0..5 {
        for j in 0..6 {
            let v = 6 * i + j;
            if tbl[i][j] == tbl[i + 1][j]  {
                uf.unite(v, v + 6);
            }
        }
    }
    for i in 0..6 {
        for j in 0..5 {
            let v = 6 * i + j;
            if tbl[i][j] == tbl[i][j + 1]  {
                uf.unite(v, v + 1);
            }
        }
    }
    let mut r = vec![];
    for i in 0..36 {
        if uf.root(i) == i {
            r.push(i);
        }
    }
    r.len() == 2
}

fn main() {
    input! {
        a: [[i32; 4]; 4],
    }
    let mut mask = 0;
    for i in 0..4 {
        for j in 0..4 {
            if a[i][j] == 1 {
                mask |= 1 << (4 * i + j);
            }
        }
    }
    let mut tot = 0;
    for bits in 0..1 << 16 {
        if (bits & mask) != mask {
            continue;
        }
        // connected && has no holes
        if ok(bits) {
            tot += 1;
        }
    }
    println!("{}", tot);
}
