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

// skew heap: http://hos.ac/blog/#blog0001
#[derive(Clone)]
struct SkewHeap<T> {
    l: Option<Box<Self>>,
    r: Option<Box<Self>>,
    val: T,
}

impl<T: PartialOrd> SkewHeap<T> {
    fn new() -> Option<Box<Self>> {
        None
    }
    fn with_one(a: T) -> Option<Box<Self>> {
        Some(Box::new(SkewHeap {
            l: None,
            r: None,
            val: a,
        }))
    }
    fn peek(a: &Option<Box<Self>>) -> Option<&T> {
        if let Some(a) = a.as_ref() {
            Some(&a.val)
        } else {
            None
        }
    }
    // Complexity: amortized O(log (|a| + |b|))
    fn meld(a: Option<Box<Self>>, b: Option<Box<Self>>) -> Option<Box<Self>> {
        let (mut a, mut b) = match (a, b) {
            (None, b) => return b,
            (a, None) => return a,
            (Some(a), Some(b)) => (a, b)
        };
        if a.val > b.val {
            std::mem::swap(&mut a, &mut b);
        }
        a.r = Self::meld(a.r, Some(b));
        std::mem::swap(&mut a.l, &mut a.r);
        Some(a)
    }
    fn pop(h: &mut Option<Box<Self>>) -> Option<T> {
        if let Some(inner) = std::mem::replace(h, None) {
            let new = Self::meld(inner.l, inner.r);
            let ret = Some(inner.val);
            *h = new;
            return ret;
        }
        None
    }
    fn push(h: &mut Option<Box<Self>>, t: T) {
        let sing = Self::with_one(t);
        *h = Self::meld(std::mem::replace(h, None), sing);
    }
}

// https://ei1333.github.io/luzhiled/snippets/dp/hu-tucker.html
fn hu_tucker(a: &[i64]) -> i64 {
    if a.is_empty() {
        return 0;
    }
    let inf = std::i64::MAX / 2;
    let n = a.len();
    let mut a = a.to_vec();
    type Heap = SkewHeap<i64>;
    // Invariant: hs[i] contains information of merged nodes in i < x < rs[i].
    let mut hs = vec![Heap::new(); n - 1];
    let mut ls = vec![0; n];
    // Invariant: rs[i] < n <====> i represents an unmerged leaf node.
    let mut rs = vec![0; n - 1];
    // Invariant: cs[i] is the minimum value among:
    // - a[i] + a[rs[i]]
    // - a[i] + min hs[i]
    // - a[rs[i]] + min hs[i]
    // - the sum of the two least elements in hs[i]
    let mut cs = vec![0; n - 1];
    let mut que = BinaryHeap::new();
    for i in 0..n - 1 {
        ls[i + 1] = i;
        rs[i] = i + 1;
        cs[i] = a[i] + a[i + 1];
        que.push((Reverse(cs[i]), i));
    }
    let mut ret = 0;
    for _ in 0..n - 1 {
        let (c, mut i) = loop {
            let (Reverse(c), i) = que.pop().unwrap();
            if rs[i] < n && cs[i] == c {
                break (c, i);
            }
        };

        let mut ml = false;
        let mut mr = false;
        let mut picked = false;
        if let Some(&top1) = Heap::peek(&hs[i]) {
            if a[i] + top1 == c {
                Heap::pop(&mut hs[i]);
                ml = true;
                picked = true;
            }
        }
        if !picked && a[i] + a[rs[i]] == c {
            ml = true;
            mr = true;
            picked = true;
        }
        if !picked {
            let top1 = Heap::pop(&mut hs[i]).unwrap();
            if let Some(&top2) = Heap::peek(&hs[i]) {
                if top2 + top1 == c {
                    Heap::pop(&mut hs[i]);
                    picked = true;
                }
            }
            if !picked {
                mr = true;
            }
        }
        ret += c;
        Heap::push(&mut hs[i], c);
        if ml {
            a[i] = inf;
        }
        if mr {
            a[rs[i]] = inf;
        }
        if ml && i > 0 {
            let j = ls[i];
            hs[j] = Heap::meld(std::mem::replace(&mut hs[j], Heap::new()),
                               std::mem::replace(&mut hs[i], Heap::new()));
            rs[j] = rs[i];
            rs[i] = n;
            ls[rs[j]] = j;
            i = j;
        }
        if mr && rs[i] + 1 < n {
            let j = rs[i];
            hs[i] = Heap::meld(std::mem::replace(&mut hs[i], Heap::new()),
                               std::mem::replace(&mut hs[j], Heap::new()));
            rs[i] = rs[j];
            rs[j] = n;
            ls[rs[i]] = i;
        }
        cs[i] = a[i] + a[rs[i]];
        if let Some(top) = Heap::pop(&mut hs[i]) {
            cs[i] = min(cs[i], min(a[i], a[rs[i]]) + top);
            if let Some(&top2) = Heap::peek(&hs[i]) {
                cs[i] = min(cs[i], top + top2);
            }
            Heap::push(&mut hs[i], top);
        }
        que.push((Reverse(cs[i]), i));
    }
    ret
}

// Tags: hu-tucker, meldable-heap, skew-heap, optimal-alphabetic-binary-trees
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        a: [i64; n],
    }
    puts!("{}\n", hu_tucker(&a));
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
