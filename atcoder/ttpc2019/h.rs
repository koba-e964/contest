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

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}

// Quick-Find data structure.
// Verified by: https://atcoder.jp/contests/cf17-tournament-round3-open/submissions/22928265
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

// http://www.prefield.com/algorithm/container/avl_tree.html
#[derive(Clone)]
struct Node {
    p: i64,
    x: i64,
    xsum: i64,
    pxsum: i64,
    size: usize,
    height: i32,
    ch: [Option<Box<Node>>; 2],
}

impl Node {
    fn new(p: i64, x: i64) -> Self {
        Node {
            p: p,
            x: x,
            xsum: x,
            pxsum: p * x,
            size: 1,
            height: 1,
            ch: [None, None],
        }
    }
    // Insert a single element represented as a leaf node.
    fn insert(l: Option<Box<Node>>, x: Node) -> Option<Box<Node>> {
        let mut t = match l {
            None => return Some(Box::new(x)),
            Some(l) => l
        };
        if t.p <= x.p {
            t.ch[0] = Node::insert(t.ch[0].take(), x);
        } else {
            t.ch[1] = Node::insert(t.ch[1].take(), x);
        }
        Node::cons(&mut t);
        Some(Node::balance(t))
    }
    fn len(t: &Option<Box<Node>>) -> usize {
        if let &Some(ref x) = t {
            x.size
        } else {
            0
        }
    }
    fn ht(t: &Option<Box<Node>>) -> i32 {
        if let &Some(ref x) = t {
            x.height
        } else {
            0
        }
    }
    fn xsum(t: &Option<Box<Node>>) -> i64 {
        if let &Some(ref x) = t {
            x.xsum
        } else {
            0
        }
    }
    fn pxsum(t: &Option<Box<Node>>) -> i64 {
        if let &Some(ref x) = t {
            x.pxsum
        } else {
            0
        }
    }
    // Make t consistent.
    fn cons(t: &mut Box<Self>) {
        t.xsum = t.x;
        t.pxsum = t.p * t.x;
        for i in 0..2 {
            if let &Some(ref c) = &t.ch[i] {
                t.xsum += c.xsum;
                t.pxsum = t.pxsum.saturating_add(c.pxsum);
                assert!(if i == 0 { t.p <= c.p } else { t.p >= c.p }, "i = {}. t.p = {}, c.p = {}", i, t.p, c.p);
            }
        }
        t.height = max(Node::ht(&t.ch[0]), Node::ht(&t.ch[1])) + 1;
        t.size = 1 + Node::len(&t.ch[0]) + Node::len(&t.ch[1]);
    }
    fn rotate(mut t: Box<Self>, l: usize, r: usize) -> Box<Self> {
        let mut s = t.ch[r].take().unwrap();
        t.ch[r] = s.ch[l].take();
        Node::cons(&mut t);
        s.ch[l] = Some(Self::balance(t));
        Node::cons(&mut s);
        Self::balance(s)
    }
    fn balance(mut t: Box<Self>) -> Box<Self> {
        for i in 0..2 {
            if Self::ht(&t.ch[1 - i]) - Self::ht(&t.ch[i]) < -1 {
                let tmp = t.ch[i].as_mut().unwrap();
                if Self::ht(&tmp.ch[1 - i]) - Self::ht(&tmp.ch[i]) > 0 {
                    *tmp = 
                        Self::rotate(std::mem::replace(tmp, Box::new(Node::new(0, 0))), i, 1 - i);
                }
                return Self::rotate(t, 1 - i , i);
            }
        }
        Node::cons(&mut t);
        t
    }
    fn each<F: FnMut((i64, i64))>(t: Option<Box<Self>>, f: &mut F) {
        let mut t = match t {
            None => return,
            Some(t) => t,
        };
        Node::each(t.ch[0].take(), f);
        f((t.p, t.x));
        Node::each(t.ch[1].take(), f);
    }
    fn profit(t: &Option<Box<Self>>, mut s: i64) -> i64 {
        if t.is_none() {
            return 0;
        }
        if Node::xsum(&t) <= s {
            return Node::pxsum(&t);
        }
        let t = t.as_ref().unwrap();
        let mut ans = 0;
        if Node::xsum(&t.ch[0]) >= s {
            return Node::profit(&t.ch[0], s);
        }
        ans += Node::pxsum(&t.ch[0]);
        s -= Node::xsum(&t.ch[0]);
        let r = min(s, t.x);
        ans += t.p * r;
        s -= r;
        ans += Node::profit(&t.ch[1], s);
        ans
    }
}

#[derive(Clone)]
struct AVLTree {
    root: Option<Box<Node>>,
}

impl AVLTree {
    fn new() -> Self {
        AVLTree { root: None }
    }
    fn len(&self) -> usize {
        Node::len(&self.root)
    }
    fn insert(&mut self, p: i64, x: i64) {
        self.root = Node::insert(self.root.take(), Node::new(p, x));
    }
    // The order is not specified.
    fn each<F: FnMut((i64, i64))>(self, mut f: F) {
        Node::each(self.root, &mut f)
    }
    fn profit(&self, s: i64) -> i64 {
        Node::profit(&self.root, s)
    }
}

// O(min(|a|, |b|))
fn merge(mut a: AVLTree, mut b: AVLTree) -> AVLTree {
    if a.len() < b.len() {
        std::mem::swap(&mut a, &mut b);
    }
    b.each(|(p, x)| a.insert(p, x));
    a
}

// The maximum happiness realizable
fn calc(a: &AVLTree, lim: i64) -> i64 {
    a.profit(lim)
}

// Tags: balanced-binary-tree, avl-tree, quick-find
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        xp: [(i64, i64); n],
        q: usize,
        ab: [(usize1, usize1); q],
    }
    let mut qf = QuickFind::new(n);
    let mut dp = vec![Some(AVLTree::new()); n];
    let mut lim = vec![0i64; n];
    for i in 0..n {
        let (x, p) = xp[i];
        if x < 0 {
            let mut tmp = AVLTree::new();
            tmp.insert(p, -x);
            dp[i] = Some(tmp);
        } else {
            lim[i] = x;
        }
    }
    let mut tot: i64 = 0;
    for &(a, b) in &ab {
        if !qf.is_same_set(a, b) {
            let ra = qf.root(a);
            let rb = qf.root(b);
            qf.unite(a, b);
            let r = qf.root(a);
            assert!(ra == r || rb == r);
            let ro = ra + rb - r;
            let oldr = calc(&dp[r].as_ref().unwrap(), lim[r]);
            let oldro = calc(&dp[ro].as_ref().unwrap(), lim[ro]);
            dp[r] = Some(merge(dp[r].take().unwrap(), dp[ro].take().unwrap()));
            lim[r] += lim[ro];
            lim[ro] = 0;
            let newr = calc(&dp[r].as_ref().unwrap(), lim[r]);
            tot += newr - oldr - oldro;
        }
        puts!("{}\n", tot);
    }
}
