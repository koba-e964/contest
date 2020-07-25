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
    ($next:expr, [graph1; $len:expr]) => {{
        let mut g = vec![vec![]; $len];
        let ab = read_value!($next, [(usize1, usize1)]);
        for (a, b) in ab {
            g[a].push(b);
            g[b].push(a);
        }
        g
    }};
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
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

const INF: usize = 1 << 28;
const W: usize = 200_001;

fn coll_head(plus: &[(usize, usize)], minus: &[(usize, usize)]) -> usize {
    let mut ptbl = vec![vec![]; W];
    let mut mtbl = vec![vec![]; W];
    for &(x, y) in plus {
        ptbl[y].push(x);
    }
    for &(x, y) in minus {
        mtbl[y].push(x);
    }
    let mut mi = INF;
    for i in 0..W {
        if ptbl[i].is_empty() || mtbl[i].is_empty() {
            continue;
        }
        ptbl[i].sort();
        mtbl[i].sort();
        for &p in &ptbl[i] {
            let idx = mtbl[i].lower_bound(&p);
            if idx < mtbl[i].len() {
                mi = min(mi, mtbl[i][idx] - p);
            }
        }
    }
    mi * 5
}
// right: (a, b)
// down: (d, W - c) = (e, f)
// collide <=> a + b = c + d && d < b (t = b - d = c - a)
fn coll_side(right: &[(usize, usize)], down: &[(usize, usize)]) -> usize {
    let mut rtbl = vec![vec![]; 2 * W];
    let mut dtbl = vec![vec![]; 2 * W];
    let mut mi = INF;
    for &(x, y) in right {
        rtbl[x + y].push(y);
    }
    for &(x, y) in down {
        let c = W - y;
        let d = x;
        dtbl[c + d].push(d);
    }
    for i in 0..2 * W {
        rtbl[i].sort();
        dtbl[i].sort();
        for &b in &rtbl[i] {
            let idx = dtbl[i].lower_bound(&b);
            if idx > 0 {
                let dist = b - dtbl[i][idx - 1];
                mi = min(mi, dist);
            }
        }
    }
    mi * 10
}
    
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize,
        xyu: [(usize, usize, chars); n],
    }
    let mut dirs = vec![vec![]; 4];
    for &(x, y, ref u) in &xyu {
        let u = u.iter().cloned().collect::<String>();
        let idx = ["R", "U", "L", "D"].iter().position(|&dir| dir == u).unwrap();
        dirs[idx].push((x, y));
    }
    let mut mi = INF;
    mi = min(mi, coll_head(&dirs[0], &dirs[2]));
    for &v in &[1, 3] {
        for j in 0..dirs[v].len() {
            let (x, y) = dirs[v][j];
            dirs[v][j] = (y, x);
        }
    }
    mi = min(mi, coll_head(&dirs[1], &dirs[3]));
    for &v in &[1, 3] {
        for j in 0..dirs[v].len() {
            let (x, y) = dirs[v][j];
            dirs[v][j] = (y, x);
        }
    }
    // TODO
    for v in 1..4 {
        for i in 0..dirs[v].len() {
            for _ in 0..v {
                let (x, y) = dirs[v][i];
                dirs[v][i] = (y, W - x);
            }
        }
    }
    for i in 0..4 {
        mi = min(mi, coll_side(&dirs[i], &dirs[(i + 1) % 4]));
    }
    if mi < INF {
        puts!("{}\n", mi);
    } else {
        puts!("SAFE\n");
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
