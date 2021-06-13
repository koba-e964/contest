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

// Quick-Find data structure.
// Verified by: https://atcoder.jp/contests/cf17-tournament-round3-open/submissions/22928265
// Verified by: https://atcoder.jp/contests/ttpc2019/submissions/23384553 (polymorphic version)
struct QuickFind<T = ()> {
    root: Vec<usize>, mem: Vec<Vec<usize>>,
    dat: Vec<T>, default: T,
}

impl QuickFind<()> {
    #[allow(unused)]
    fn new(n: usize) -> Self {
        Self::with_dat(n, ())
    }
    #[allow(unused)]
    fn unite(&mut self, x: usize, y: usize) {
        self.unite_with_hooks(x, y, |&(), _| (), |(), ()| ());
    }
}
impl<T: Clone> QuickFind<T> {
    fn with_dat(n: usize, def: T) -> Self {
        let root = (0..n).collect();
        let mut mem = vec![vec![]; n];
        for i in 0..n {
            mem[i] = vec![i];
        }
        QuickFind { root: root, mem: mem, dat: vec![def.clone(); n], default: def }
    }
    fn root(&self, x: usize) -> usize {
        self.root[x]
    }
    #[allow(unused)]
    fn get(&self, idx: usize) -> &T {
        &self.dat[self.root[idx]]
    }
    #[allow(unused)]
    fn set(&mut self, idx: usize, val: T) {
        let r = self.root[idx];
        self.dat[r] = val;
    }
    // unite always merges y to x if |x| >= |y|.
    fn unite_with_hooks<F: FnMut(&T, i64), G: FnMut(T, T) -> T>(
        &mut self, x: usize, y: usize,
        mut hook: F, mut merge: G) {
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
        // hook
        hook(&self.dat[x], -1);
        hook(&self.dat[y], -1);
        self.dat[x] = merge(
            std::mem::replace(&mut self.dat[x], self.default.clone()),
            std::mem::replace(&mut self.dat[y], self.default.clone()),
        );
        hook(&self.dat[x], 1);
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
    let mut uf = QuickFind::with_dat(n, (vec![], 0, 0));
    for i in 0..n {
        uf.set(i, (vec![i], INF, 0));
    }
    const INF: i64 = 1 << 40;
    let mut ans = vec![0; n];
    for (a, b, c) in abc {
        if !uf.is_same_set(a, b) {
            uf.unite_with_hooks(
                a, b,
                |_, _| {},
                |(mut v1, mi1, la1), (v2, mi2, la2)| {
                    assert!(v1.len() >= v2.len());
                    let del1 = (v1.len() - 1) as i64 * (mi1 - c);
                    let del2 = (v2.len() - 1) as i64 * (mi2 - c);
                    for &v in &v2 {
                        ans[v] += del2 + la2 - la1 - del1;
                    }
                    v1.extend_from_slice(&v2);
                    (v1, c, la1 + del1)
                });
        }
    }
    assert_eq!(uf.size(0), n);
    for i in 0..n {
        let &(_, mi, la) = uf.get(i);
        ans[i] += la + mi * (n as i64 - 1);
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
