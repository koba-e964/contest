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

fn main() {
    input! {
        h: usize, w: usize, n: usize,
        s: (usize1, usize1), g: (usize1, usize1),
        xyk: [(usize1, usize1, usize); n],
    }
    let mut imos = vec![vec![0; w + 2]; h + 2];
    for &(x, y, k) in &xyk {
        imos[x - k][y] += 1;
        if y > k {
            imos[x + 1][y - k - 1] -= 1;
        }
        imos[x + 1][y + k + 1] -= 1;
        imos[x + k + 2][y] += 1;
        if k > 0 {
            imos[x - k + 1][y] += 1;
            imos[x + 1][y - k] -= 1;
            imos[x + 1][y + k] -= 1;
            imos[x + k + 1][y] += 1;
        }
    }
    for i in 0..h + 1 {
        for j in 0..w + 2 {
            if j > 0 {
                imos[i + 1][j - 1] += imos[i][j];
            }
            if j < w + 1 {
                imos[i + 1][j + 1] += imos[i][j];
            }
            if i < h {
                imos[i + 2][j] -= imos[i][j];
            }
        }
    }
    let mut uf = UnionFind::new(h * w);
    for i in 0..h - 1 {
        for j in 0..w {
            let v = i * w + j;
            if imos[i][j] == 0 && imos[i + 1][j] == 0 {
                uf.unite(v, v + w);
            }
        }
    }
    for i in 0..h {
        for j in 0..w - 1 {
            let v = i * w + j;
            if imos[i][j] == 0 && imos[i][j + 1] == 0 {
                uf.unite(v, v + 1);
            }
        }
    }
    println!("{}", if uf.is_same_set(s.0 * w + s.1, g.0 * w + g.1) {
        "Yes"
    } else {
        "No"
    });
}
