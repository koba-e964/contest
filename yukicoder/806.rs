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
 * Manages a tree and calculates the diameter of it.
 * Verified by: NJPC 2017-E
 *              (http://njpc2017.contest.atcoder.jp/submissions/1089492)
 */
struct Diameter {
    n: usize,
    edges: Vec<Vec<(usize, i64)>>,
    x: usize,
    y: usize,
}

impl Diameter {
    fn dfs(&self, v: usize, dist: &mut [i64], p: Option<usize>, d: i64) {
        dist[v] = d;
        for &(w, c) in self.edges[v].iter() {
            if Some(w) == p { continue; }
            self.dfs(w, dist, Some(v), d + c);
        }
    }
    pub fn new(n: usize) -> Self {
        Diameter {
            n: n,
            edges: vec![Vec::new(); n],
            x: n,
            y: n,
        }
    }
    pub fn add_edge(&mut self, a: usize, b: usize, c: i64) {
        self.edges[a].push((b, c));
        self.edges[b].push((a, c));
    }
    pub fn diameter(&mut self) -> (usize, usize) {
        let n = self.n;
        if self.x < n {
            return (self.x, self.y);
        }
        // farthest from 0
        let mut dist = vec![-1; n];
        self.dfs(0, &mut dist, None, 0);
        let mut maxi = 0;
        for i in 1 .. n {
            if dist[maxi] < dist[i] {
                maxi = i;
            }
        }
        self.x = maxi;
        // farthest from x
        self.dfs(maxi, &mut dist, None, 0);
        let mut maxi = 0;
        for i in 0 .. n {
            if dist[maxi] < dist[i] {
                maxi = i;
            }
        }
        self.y = maxi;
        (self.x, self.y)
    }
    pub fn farthest(&mut self) -> Vec<i64> {
        let n = self.n;
        if self.x >= n {
            self.diameter();
        }
        let mut ret = vec![0; n];
        let mut tmp = vec![-1; n];
        /* For every vertex, the farthest point from it is either x or y. */
        self.dfs(self.x, &mut ret, None, 0);
        self.dfs(self.y, &mut tmp, None, 0);
        for i in 0 .. n {
            ret[i] = max(ret[i], tmp[i]);
        }
        ret
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
        ab: [(usize1, usize1); n - 1],
    }
    let mut diam = Diameter::new(n);
    let mut deg = vec![0; n];
    for (a, b) in ab {
        diam.add_edge(a, b, 1);
        deg[a] += 1;
        deg[b] += 1;
    }
    let mut tot = 0;
    for i in 0..n {
        if deg[i] >= 2 {
            tot += deg[i] - 2;
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
