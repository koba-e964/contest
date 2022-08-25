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

type Cont = BTreeSet<i32>;

fn dfs(v: usize, a: &[i32], b: &[i32]) -> Option<Cont> {
    fn add_ints(v: &mut Cont, a: Option<Cont>, b: Option<Cont>) {
        match (a, b) {
            (None, Some(b)) => {
                if v.is_empty() {
                    *v = b;
                } else {
                    for b in b { v.insert(b); }
                }
            }
            (Some(a), None) => {
                if v.is_empty() {
                    *v = a;
                } else {
                    for a in a { v.insert(a); }
                }
            }
            (Some(mut a), Some(mut b)) => {
                if a.len() > b.len() {
                    std::mem::swap(&mut a, &mut b);
                }
                for k in a.intersection(&b) {
                    v.insert(*k);
                }
            }
            (None, None) => panic!(),
        }
    }
    assert_eq!(a.len(), b.len());
    if a.is_empty() {
        return None;
    }
    if v == 0 {
        return Some(vec![a[0] ^ b[0]].into_iter().collect());
    }
    let mut a0 = 0;
    let mut b0 = 0;
    let mut b1 = 0;
    for &a in a {
        if (a & 1 << (v - 1)) == 0 {
            a0 += 1;
        }
    }
    for &b in b {
        if (b & 1 << (v - 1)) == 0 {
            b0 += 1;
        } else {
            b1 += 1;
        }
    }
    let mut ans = Cont::new();
    if a0 == b0 {
        let sub0 = dfs(v - 1, &a[..a0], &b[..b0]);
        let sub1 = dfs(v - 1, &a[a0..], &b[b0..]);
        add_ints(&mut ans, sub0, sub1);
    }
    if a0 == b1 {
        let sub0 = dfs(v - 1, &a[..a0], &b[b0..]);
        let sub1 = dfs(v - 1, &a[a0..], &b[..b0]);
        add_ints(&mut ans, sub0, sub1);
    }
    Some(ans)
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        a: [i32; n],
        b: [i32; n],
    }
    let mut a = a;
    let mut b = b;
    a.sort();
    b.sort();
    let mut ans: Vec<i32> = dfs(30, &a, &b).unwrap().into_iter().collect();
    ans.sort();
    puts!("{}\n", ans.len());
    for a in ans {
        puts!("{}\n", a);
    }
}
