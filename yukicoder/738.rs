use std::cmp::*;
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

macro_rules! read_value {
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

// http://www.prefield.com/algorithm/container/avl_tree.html
// https://atcoder.jp/contests/ttpc2019/tasks/ttpc2019_h
// https://atcoder.jp/contests/ttpc2019/submissions/23366439
#[derive(Clone)]
struct Node {
    p: i64,
    sum: i64,
    size: usize,
    height: i32,
    ch: [Option<Box<Node>>; 2],
}

impl Node {
    fn new(p: i64) -> Self {
        Node {
            p: p,
            sum: p,
            size: 1,
            height: 1,
            ch: [None, None],
        }
    }
    // Insert a single element represented as a leaf node.
    fn insert(l: Option<Box<Node>>, x: Node) -> Option<Box<Node>> {
        let mut t = match l {
            None => return Some(Box::new(x)),
            Some(l) => l,
        };
        if t.p <= x.p {
            t.ch[0] = Node::insert(t.ch[0].take(), x);
        } else {
            t.ch[1] = Node::insert(t.ch[1].take(), x);
        }
        Node::cons(&mut t);
        Some(Node::balance(t))
    }
    fn erase(l: Option<Box<Node>>, p: i64) -> Option<Box<Node>> {
        let mut t = match l {
            None => return None,
            Some(l) => l,
        };
        if t.p == p {
            return Node::move_down(t.ch[0].take(), t.ch[1].take());
        }
        if t.p < p {
            t.ch[0] = Node::erase(t.ch[0].take(), p);
        } else {
            t.ch[1] = Node::erase(t.ch[1].take(), p);
        }
        Node::cons(&mut t);
        Some(Node::balance(t))
    }
    fn move_down(l: Option<Box<Node>>, r: Option<Box<Node>>) -> Option<Box<Node>> {
        let mut t = match l {
            None => return r,
            Some(l) => l
        };
        t.ch[1] = Node::move_down(t.ch[1].take(), r);
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
    fn sum(t: &Option<Box<Node>>) -> i64 {
        if let &Some(ref x) = t {
            x.sum
        } else {
            0
        }
    }
    fn at(l: &Option<Box<Node>>, pos: usize) -> Option<i64> {
        let t = match l {
            None => return None,
            Some(l) => l,
        };
        if pos >= t.size {
            return None;
        }
        let ll = Node::len(&t.ch[0]);
        if pos < ll {
            return Node::at(&t.ch[0], pos);
        }
        if pos == ll {
            return Some(t.p);
        }
        Node::at(&t.ch[1], pos - ll - 1)
    }
    fn acc(l: &Option<Box<Node>>, pos: usize) -> i64 {
        let t = match l {
            None => return 0,
            Some(l) => l,
        };
        if pos >= t.size {
            return t.sum;
        }
        let ll = Node::len(&t.ch[0]);
        if pos <= ll {
            return Node::acc(&t.ch[0], pos);
        }
        let ls = Node::sum(&t.ch[0]);
        ls + t.p + Node::acc(&t.ch[1], pos - ll - 1)
    }
    // Make t consistent.
    fn cons(t: &mut Box<Self>) {
        for i in 0..2 {
            if let &Some(ref c) = &t.ch[i] {
                assert!(if i == 0 { t.p <= c.p } else { t.p >= c.p }, "i = {}. t.p = {}, c.p = {}", i, t.p, c.p);
            }
        }
        t.height = std::cmp::max(Node::ht(&t.ch[0]), Node::ht(&t.ch[1])) + 1;
        t.size = 1 + Node::len(&t.ch[0]) + Node::len(&t.ch[1]);
        t.sum = t.p + Node::sum(&t.ch[0]) + Node::sum(&t.ch[1]);
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
                        Self::rotate(std::mem::replace(tmp, Box::new(Node::new(0))), i, 1 - i);
                }
                return Self::rotate(t, 1 - i , i);
            }
        }
        Node::cons(&mut t);
        t
    }
    fn each<F: FnMut(i64)>(t: &Option<Box<Self>>, f: &mut F) {
        let t = match t {
            None => return,
            Some(t) => t,
        };
        Node::each(&t.ch[0], f);
        f(t.p);
        Node::each(&t.ch[1], f);
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
    #[allow(unused)]
    fn len(&self) -> usize {
        Node::len(&self.root)
    }
    fn insert(&mut self, p: i64) {
        self.root = Node::insert(self.root.take(), Node::new(p));
    }
    fn at(&self, pos: usize) -> Option<i64> {
        Node::at(&self.root, pos)
    }
    fn acc(&self, pos: usize) -> i64 {
        Node::acc(&self.root, pos)
    }
    fn erase(&mut self, p: i64) {
        self.root = Node::erase(self.root.take(), p);
    }
    // The order is not specified.
    #[allow(unused)]
    fn each<F: FnMut(i64)>(&self, mut f: F) {
        Node::each(&self.root, &mut f)
    }
}

// Tags: balanced-binary-search-tree
fn main() {
    let n;
    let k;
    let mut a;
    if true {
        input! {
            n_: usize,
            k_: usize,
            a_: [i64; n_],
        }
        n = n_;
        k = k_;
        a = a_;
    } else {
        n = 100000;
        k = 50000;
        a = vec![0; n];
        for i in 0 .. n {
            a[i] = i as i64 + 1;
        }
    }
    let mut acc = vec![0; n + 1];
    for i in 0..n {
        acc[i + 1] = acc[i] + a[i];
    }
    let mut ms = AVLTree::new();
    for i in 0..k {
        ms.insert(a[i]);
    }
    const INF: i64 = 1 << 50;
    let mut mi = INF;
    for i in k..n + 1 {
        let mid = (k + 1) / 2;
        let med = ms.at(mid - 1).unwrap();
        let sum = ms.acc(mid);
        let tmp = sum * 2 - acc[i] + acc[i - k] + med * (k as i64 - mid as i64 * 2);
        // eprintln!("{} => {} (med = {}, sum = {})", i, tmp, med, sum);
        // ms.each(|x| eprint!(" {}", x));
        // eprintln!();
        mi = min(mi, tmp);
        if i < n {
            ms.erase(a[i - k]);
            ms.insert(a[i]);
        }
    }
    println!("{}", mi);
}
