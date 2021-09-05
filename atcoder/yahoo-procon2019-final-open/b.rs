use std::cmp::*;
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
            ret[i] = std::cmp::max(ret[i], tmp[i]);
        }
        ret
    }
}

trait Bisect<T> {
    fn lower_bound(&self, val: &T) -> usize;
    fn upper_bound(&self, val: &T) -> usize;
}

impl<T: Ord> Bisect<T> for [T] {
    fn lower_bound(&self, val: &T) -> usize {
        let mut pass = self.len() + 1;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if &self[mid - 1] >= val {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        pass - 1
    }
    fn upper_bound(&self, val: &T) -> usize {
        let mut pass = self.len() + 1;
        let mut fail = 0;
        while pass - fail > 1 {
            let mid = (pass + fail) / 2;
            if &self[mid - 1] > val {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        pass - 1
    }
}

fn get(n: usize, e: &[(usize, usize)]) -> Vec<i64> {
    let mut diam = Diameter::new(n);
    for &(a, b) in e {
        diam.add_edge(a, b, 1);
    }
    diam.farthest()
}

// Tags: diameter, farthest-points-on-trees
fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

fn solve() {
    input! {
        n: usize,
        ea: [(usize1, usize1); n - 1],
        m: usize,
        eb: [(usize1, usize1); m - 1],
    }
    let mut fa = get(n, &ea);
    let mut fb = get(m, &eb);
    fa.sort();
    fb.sort();
    let k = max(fa[n - 1], fb[m - 1]);
    let mut acc = vec![0; m + 1];
    for i in 0..m {
        acc[i + 1] = acc[i] + fb[i];
    }
    let mut tot = 0;
    for i in 0..n {
        let idx = fb.lower_bound(&(k - fa[i] - 1));
        tot += (m - idx) as i64 * (fa[i] + 1) + acc[m] - acc[idx];
        tot += idx as i64 * k;
    }
    println!("{}", tot);
}
