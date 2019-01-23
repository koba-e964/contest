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

/**
 * Union-Find tree.
 * Verified by yukicoder No.94 (http://yukicoder.me/submissions/82111)
 */
struct UnionFind { disj: Vec<usize>, rank: Vec<usize> }

impl UnionFind {
    fn new(n: usize) -> Self {
        let disj = (0..n).collect();
        UnionFind { disj: disj, rank: vec![0; n] }
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
        self.rank[y] = max(self.rank[y], self.rank[x] + 1);
    }
    fn is_same_set(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
}

const INF: i64 = 1 << 50;

fn upd(p: &mut [(i64, usize, usize); 2], val: (i64, usize, usize)) {
    if p[0] > val && p[0].1 != val.1 {
        p[1] = p[0];
        p[0] = val;
        return;
    }
    if p[0].1 == val.1 {
        if p[0] > val {
            p[0] = val;
        }
        return;
    }
    if p[1] > val {
        p[1] = val;
    }
    assert!(p[0].0 == INF || p[1].0 == INF || p[0].1 != p[1].1);
    assert!(p[0] < p[1]);
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        d: i64,
        a: [i64; n],
    }
    /*
    let n = 200000;
    let d = 1100;
    let mut a = vec![1; n];
    let mut x = 12525i64;
    for i in 0 .. n {
        a[i] = x;
    	x = (x * 1555 + 2624737) % 1000000007;
    }
     */
    let mut tot = 0;
    let mut conn = n;
    let mut uf = UnionFind::new(n);
    let mut lft = vec![0; n];
    let mut rgt = vec![0; n];
    for i in 0 .. n {
        lft[i] = a[i] - i as i64 * d;
        rgt[i] = a[i] + i as i64 * d;
    }
    let mut mi = vec![(INF, n * n); n];
    while conn > 1 {
        // eprintln!("conn = {}", conn);
        for i in 0 .. n { mi[i] = (INF, n * n); }
        let mut col = vec![0; n];
        for i in 0 .. n { col[i] = uf.root(i); }
        let mut pool: [(i64, usize, usize); 2] = [(INF, n, n); 2];
        for i in 0 .. n {
            if i > 0 {
                let tmp;
                if col[i] != col[pool[0].1] {
                    tmp = (pool[0].0 + rgt[i], pool[0].2 * n + i);
                } else {
                    tmp = (pool[1].0 + rgt[i], pool[1].2 * n + i);
                }
                mi[col[i]] = min(mi[col[i]], tmp);
            }
            upd(&mut pool, (lft[i], col[i], i));
        }
        pool = [(INF, n, n); 2];
        for i in (0 .. n).rev() {
            if i + 1 < n {
                let tmp;
                if col[i] != col[pool[0].1] {
                    tmp = (pool[0].0 + lft[i], pool[0].2 + n * i);
                } else {
                    tmp = (pool[1].0 + lft[i], pool[1].2 + n * i);
                }
                mi[col[i]] = min(mi[col[i]], tmp);
            }
            upd(&mut pool, (rgt[i], col[i], i));
        }
        for i in 0 .. n {
            if mi[i].0 < INF {
                let x = mi[i].1 / n;
                let y = mi[i].1 % n;
                if !uf.is_same_set(x, y) {
                    uf.unite(x, y);
                    tot += mi[i].0;
                    conn -= 1;
                }
            }
        }
    }
    puts!("{}\n", tot);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
