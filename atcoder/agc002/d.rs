#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
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

/*
 * Persistent Union Find tree.
 * Reference: https://misteer.hatenablog.com/entry/persistentUF
 */
const INF: i32 = 1 << 28;
struct PersistentUnionFind {
    par: Vec<usize>,
    time: Vec<i32>,
    rank: Vec<i32>,
    num: Vec<Vec<(i32, usize)>>, // [(time, size of this component)]
    now: i32,
}
impl PersistentUnionFind {
    fn new(n: usize) -> Self {
        let mut par = vec![0; n];
        let time = vec![INF; n];
        let rank = vec![0; n];
        let num = vec![vec![(-1, 1)]; n];
        for i in 0 .. n {
            par[i] = i;
        }
        PersistentUnionFind {
            par: par,
            time: time,
            rank: rank,
            num: num,
            now: 0
        }
    }
    fn root(&self, mut x: usize, t: i32) -> usize {
        loop {
            if self.time[x] > t { return x; }
            x = self.par[x];
        }
    }
    // returns the current time
    #[allow(dead_code)]
    fn unite(&mut self, x: usize, y: usize) -> i32 {
        let mut now = self.now;
        let mut x = self.root(x, now);
        let mut y = self.root(y, now);
        if x == y { return now; }
        now += 1;
        if self.rank[x] <= self.rank[y] { std::mem::swap(&mut x, &mut y); }
        self.par[y] = x;
        self.time[y] = now;
        self.rank[x] = std::cmp::max(self.rank[x], self.rank[y] + 1);
        let size0 = self.num[x].last().unwrap().1;
        let size1 = self.num[y].last().unwrap().1;
        self.num[x].push((now, size0 + size1));
        self.now = now;
        now
    }
    #[allow(dead_code)]
    fn size(&self, x: usize, t: i32) -> usize {
        let x = self.root(x, t);
        let idx = self.num[x].binary_search(&(t, usize::max_value())).err().unwrap() - 1;
        self.num[x][idx].1
    }
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

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    input!{
        n: usize,
        m: usize,
        ab: [(usize1, usize1); m],
        q: usize,
        xyz: [(usize1, usize1, usize); q],
    }
    let mut uf = PersistentUnionFind::new(n);
    let mut time = vec![0; m];
    for (i, (a, b)) in ab.into_iter().enumerate() {
        time[i] = uf.unite(a, b);
    }
    for (x, y, z) in xyz {
        let mut pass = m;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            let t = time[mid - 1];
            let rx = uf.root(x, t);
            let ry = uf.root(y, t);
            let size = if rx == ry {
                uf.size(rx, t)
            } else {
                uf.size(rx, t) + uf.size(ry, t)
            };
            if size >= z {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        puts!("{}\n", pass);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
