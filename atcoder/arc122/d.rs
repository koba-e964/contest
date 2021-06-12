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

const INF: i64 = 1 << 50;

struct Trie {
    ch: [Option<Box<Trie>>; 2],
    a: i64, // count
}

fn new() -> Trie {
    Trie {
        ch: [None, None],
        a: INF,
    }
}

fn add(t: &mut Trie, x: i64, dep: usize) {
    if dep == 0 {
        t.a += 1;
        return;
    }
    let idx = (x >> (dep - 1)) & 1;
    let idx = idx as usize;
    if t.ch[idx].is_none() {
        t.ch[idx] = Some(Box::new(new()));
    }
    add(t.ch[idx].as_mut().unwrap(), x, dep - 1);
    t.a += 1;
}

fn get(t: Option<&Trie>, x: i64, dep: usize) -> i64 {
    let t = if let Some(t) = t {
        t
    } else {
        return 0;
    };
    if dep == 0 {
        return 0;
    }
    let xbit = ((x >> (dep - 1)) & 1) as usize;
    if let &Some(ref l) = &t.ch[xbit] {
        if l.a > 0 {
            return get(Some(l), x, dep - 1);
        }
    }
    let sub = get(t.ch[xbit ^ 1].as_ref().map(|x| x.as_ref()), x, dep - 1);
    sub | 1 << (dep - 1)
}

fn rec(a: &[i64], pos: usize) -> i64 {
    if pos == 0 {
        return 0;
    }
    let n = a.len();
    if n == 0 {
        return 0;
    }
    let mut p = 0;
    while p < n && (a[p] & 1 << (pos - 1)) == 0 {
        p += 1;
    }
    if p % 2 == 0 {
        let sub1 = rec(&a[..p], pos - 1);
        let sub2 = rec(&a[p..], pos - 1);
        return max(sub1, sub2);
    }
    let mut trie = new();
    for i in 0..p {
        add(&mut trie, a[i], pos);
    }
    let mut mi = INF;
    for i in p..n {
        let x = get(Some(&trie), a[i], pos);
        mi.chmin(x);
    }
    mi
}

// Tags: trie
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
        a: [i64; 2 * n],
    }
    let mut a = a;
    a.sort();
    let ans = rec(&a, 30);
    puts!("{}\n", ans);
}
