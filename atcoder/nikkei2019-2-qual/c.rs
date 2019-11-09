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
    //($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
    ($($format:tt)*) => {};
}
#[allow(unused)]
macro_rules! debugln {
    //($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
    ($($format:tt)*) => {};
}

/**
 * Union-Find tree.
 * Verified by https://atcoder.jp/contests/keyence2019/submissions/4071067
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
        self.rank[y] = std::cmp::max(self.rank[y], self.rank[x] + 1);
    }
    #[allow(unused)]
    fn is_same_set(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
    }
    let mut ba = vec![(0, 0); n];
    for i in 0..n {
        ba[i] = (b[i], a[i]);
    }
    macro_rules! fail {
        () => {
            puts!("No\n");
            return;
        };
    }
    ba.sort();
    let mut a = vec![(0, 0); n];
    for i in 0..n {
        a[i] = (ba[i].1, i);
    }
    a.sort();
    for i in 0..n {
        if a[i].0 > ba[i].0 {
            fail!();
        }
    }
    for i in 0..n {
        debug!(" {}", ba[i].1);
    }
    debugln!("");
    debugln!("a = {:?}", a);
    // heuristics
    for i in 0..n {
        if ba[i].1 == a[i].0 {
            puts!("Yes\n");
            return;
        }
    }
    let mut uf = UnionFind::new(n);
    let mut conn = n;
    let mut que = BinaryHeap::new();
    let mut pos = 0;
    for i in 0..n {
        let mut ok = false;
        while pos < n && a[pos].0 <= ba[i].0 {
            let ind = a[pos].1;
            if uf.is_same_set(ind, i) {
                pos += 1;
                ok = true;
                break;
            }
            que.push((ba[a[pos].1].0, a[pos].1));
            pos += 1;
        }
        if !ok {
            let (_top, idx) = que.pop().unwrap();
            if uf.is_same_set(idx, i) {
                continue;
            }
            conn -= 1;
            uf.unite(idx, i);
        }
    }
    if conn >= 2 {
        puts!("Yes\n");
    } else {
        puts!("No\n");
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
