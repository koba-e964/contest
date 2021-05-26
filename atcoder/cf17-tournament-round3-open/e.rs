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

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

struct QuickFind { root: Vec<usize>, mem: Vec<Vec<usize>> }

impl QuickFind {
    fn new(n: usize) -> Self {
        let root = (0..n).collect();
        let mut mem = vec![vec![]; n];
        for i in 0..n {
            mem[i] = vec![i];
        }
        QuickFind { root: root, mem: mem }
    }
    fn root(&self, x: usize) -> usize {
        self.root[x]
    }
    // unite always merges y to x if |x| >= |y|.
    fn unite(&mut self, x: usize, y: usize) {
        let mut x = self.root(x);
        let mut y = self.root(y);
        if x == y { return }
        if self.mem[x].len() < self.mem[y].len() {
            std::mem::swap(&mut x, &mut y);
        }
        let memy = std::mem::replace(&mut self.mem[y], vec![]);
        for &v in &memy {
            self.root[v] = x;
        }
        self.mem[x].extend_from_slice(&memy);
    }
    #[allow(unused)]
    fn is_same_set(&self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
    #[allow(unused)]
    fn size(&self, x: usize) -> usize {
        let x = self.root(x);
        self.mem[x].len()
    }
}

// Tags: quick-find, chronological-order
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    #[allow(unused)]
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    input! {
        n: usize,
        abc: [(usize1, usize1, i64); n - 1],
    }
    let mut abc = abc;
    abc.sort_by_key(|&(_, _, c)| -c);
    let mut uf = QuickFind::new(n);
    const INF: i64 = 1 << 40;
    let mut mi = vec![INF; n];
    let mut lazy = vec![0; n];
    let mut ans = vec![0; n];
    for (a, b, c) in abc {
        if !uf.is_same_set(a, b) {
            let mut x = uf.root(a);
            let mut y = uf.root(b);
            if uf.size(x) < uf.size(y) {
                std::mem::swap(&mut x, &mut y);
            }
            let delx = (uf.size(x) - 1) as i64 * (mi[x] - c);
            let dely = (uf.size(y) - 1) as i64 * (mi[y] - c);
            // QuickFind merges right to left if |left| >= |right|.
            for &v in &uf.mem[y] {
                ans[v] += dely + lazy[y] - lazy[x] - delx;
            }
            uf.unite(x, y);
            let r = uf.root(x);
            assert_eq!(r, x);
            mi[x] = c;
            lazy[x] += delx;
        }
    }
    assert_eq!(uf.size(0), n);
    let r = uf.root(0);
    for i in 0..n {
        ans[i] += lazy[r] + mi[r] * (n as i64 - 1);
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
