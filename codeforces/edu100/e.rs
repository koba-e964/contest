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

trait Change {
    fn chmax(&mut self, x: Self);
    fn chmin(&mut self, x: Self);
}
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) {
        if *self < x { *self = x; }
    }
    fn chmin(&mut self, x: T) {
        if *self > x { *self = x; }
    }
}

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

fn precomp(n: usize, xy: &[(usize, usize)]) -> Option<Vec<Vec<usize>>> {
    let mut g = vec![vec![]; n];
    let mut indeg = vec![0; n];
    let mut rel = vec![false; n];
    let mut nxt = vec![n; n];
    for &(x, y) in xy {
        g[x].push(y);
        indeg[y] += 1;
        rel[x] = true;
        rel[y] = true;
        nxt[x] = y;
    }
    let mut que = Vec::new();
    for i in 0..n {
        if rel[i] && indeg[i] == 0 {
            que.push(i);
        }
    }
    let init = que.clone();
    let mut rem = (0..n).filter(|&i| rel[i]).count();
    while let Some(v) = que.pop() {
        for &w in &g[v] {
            indeg[w] -= 1;
            if indeg[w] == 0 {
                que.push(w);
            }
        }
        rem -= 1;
    }
    if rem != 0 {
        return None;
    }
    let m = init.len();
    let mut ret = vec![vec![]; m];
    for i in 0..m {
        let mut t = vec![];
        let mut v = init[i];
        while v < n {
            t.push(v);
            v = nxt[v];
        }
        ret[i] = t;
    }
    Some(ret)
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    #[allow(unused)]
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize, k: usize,
        p: [usize; n],
        xy: [(usize1, usize1); k],
    }
    let paths = if let Some(v) = precomp(n, &xy) {
        v
    } else {
        puts!("0\n");
        return;
    };
    // eprintln!("paths = {:?}", paths);
    let mut root = vec![0; n];
    let mut idx = vec![0; n];
    for i in 0..n {
        root[i] = i;
    }
    let m = paths.len();
    for i in 0..m {
        for j in 1..paths[i].len() {
            root[paths[i][j]] = paths[i][0];
            idx[paths[i][j]] = j;
        }
    }
    let mut g = vec![vec![]; n];
    let mut indeg = vec![0; n];
    for i in 0..n {
        if p[i] > 0 && root[i] != root[p[i] - 1] {
            g[p[i] - 1].push(root[i]);
            indeg[root[i]] += 1;
        }
        if p[i] > 0 && root[i] == root[p[i] - 1] {
            if idx[i] < idx[p[i] - 1] {
                puts!("0\n");
                return;
            }
        }
    }
    // eprintln!("indeg = {:?}", indeg);
    for &(x, y) in &xy {
        g[x].push(y);
        indeg[y] += 1;
    }
    let mut que = Vec::new();
    for i in 0..n {
        if indeg[i] == 0 {
            que.push(i);
        }
    }
    let mut rem = n;
    let mut ord = vec![];
    let mut invord = vec![n; n];
    while let Some(v) = que.pop() {
        // eprintln!("v = {}", v);
        invord[v] = ord.len();
        ord.push(v);
        for &w in &g[v] {
            indeg[w] -= 1;
            if indeg[w] == 0 { 
                que.push(w);
            }
        }
        rem -= 1;
    }
    let mut ok = true;
    if rem != 0 {
        ok = false;
    }
    for &(x, y) in &xy {
        if invord[x] >= n || invord[y] >= n || invord[x] + 1 != invord[y] {
            ok = false;
        }
    }
    if !ok {
        puts!("0\n");
        return;
    }
    for i in 0..n {
        ord[i] += 1;
    }
    putvec!(ord);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
