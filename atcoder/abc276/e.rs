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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
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
    input! {
        h: usize, w: usize,
        c: [chars; h],
    }
    let mut uf = UnionFind::new(h * w);
    let mut s = (0, 0);
    for i in 0..h {
        for j in 0..w {
            let v = i * w + j;
            if j > 0 && c[i][j] == '.' && c[i][j - 1] == '.' {
                uf.unite(v, v - 1);
            }
            if i > 0 && c[i][j] == '.' && c[i - 1][j] == '.' {
                uf.unite(v, v - w);
            }
            if c[i][j] == 'S' {
                s = (i, j);
            }
        }
    }
    let dxy = [(1i32, 0i32), (0, 1), (-1, 0), (0, -1)];
    let mut heads = vec![];
    for &(dx, dy) in &dxy {
        let nx = (s.0 as i32 + dx) as usize;
        let ny = (s.1 as i32 + dy) as usize;
        if nx < h && ny < w && c[nx][ny] == '.' {
            heads.push(nx * w + ny);
        }
    }
    for i in 0..heads.len() {
        for j in 0..i {
            if uf.is_same_set(heads[i], heads[j]) {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
