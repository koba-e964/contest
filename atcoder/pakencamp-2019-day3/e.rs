#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String{
            bytes
                .by_ref()
                .map(|r|r.unwrap() as char)
                .skip_while(|c|c.is_whitespace())
                .take_while(|c|!c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    }};

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

/**
 * Union-Find tree.
 * Verified by https://atcoder.jp/contests/keyence2019/submissions/4071067
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

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        h: usize, w: usize,
        s: [chars; h],
        q: usize,
        xyl: [(usize1, usize1, usize); q],
    }
    let mut acc = vec![vec![0; w + 1]; h + 1];
    for i in 0..h {
        for j in 0..w {
            acc[i + 1][j + 1] = acc[i + 1][j] + acc[i][j + 1] - acc[i][j] +
                if s[i][j] == '#' { 1 } else { 0 };
        }
    }
    let mut ma = vec![vec![0; w]; h];
    let mut queries = vec![vec![]; min(h, w) + 1];
    for i in 0..q {
        let (x, y, l) = xyl[i];
        queries[l].push((i, x, y));
    }
    let mut disc = vec![vec![]; min(h, w) + 1];
    for i in 0..h {
        for j in 0..w {
            let mut pass = 0;
            let mut fail = min(h - i, w - j) + 1;
            while fail - pass > 1 {
                let mid = (fail + pass) / 2;
                if acc[i + mid][j + mid] - acc[i][j + mid] - acc[i + mid][j]
                    + acc[i][j] == 0 {
                        pass = mid;
                    } else {
                        fail = mid;
                    }
            }
            ma[i][j] = pass;
            disc[pass].push((i, j));
        }
    }
    let mut uf = UnionFind::new(h * w);
    let dxy = [(-1, 0), (1, 0), (0, 1), (0, -1)];
    let mut ans = vec![1 << 30; q];
    for len in (1..min(h, w) + 1).rev() {
        // proc
        for &(x, y) in &disc[len] {
            for &(dx, dy) in &dxy {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx < 0 || nx >= h as i32 || ny < 0 || ny >= w as i32 {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                if ma[nx][ny] >= len {
                    uf.unite(x * w + y, nx * w + ny);
                }
            }
        }
        // query
        for &(idx, x, y) in &queries[len] {
            ans[idx] = uf.size(x * w + y);
        }
    }
    for a in ans {
        puts!("{}\n", a);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
