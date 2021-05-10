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
        n: usize, q: usize,
        s: [chars; q],
    }
    fn to_id(a: &[usize]) -> usize {
        let mut s = 0;
        for &a in a {
            s = 3 * s + a;
        }
        s
    }
    fn from_id(mut a: usize, n: usize) -> Vec<usize> {
        let mut ans = vec![0; n];
        for i in 0..n {
            ans[n - 1 - i] = a % 3;
            a /= 3;
        }
        ans
    }
    let mut p3 = 1;
    for _ in 0..n {
        p3 *= 3;
    }
    let mut que = VecDeque::new();
    for i in 0..p3 {
        let v = from_id(i, n);
        let mut t = v.clone();
        t.sort();
        if v == t {
            que.push_back((0, i));
        }
    }
    const INF: i32 = 1 << 30;
    let mut dist = vec![INF; p3];
    while let Some((d, v)) = que.pop_front() {
        if dist[v] <= d {
            continue;
        }
        dist[v] = d;
        let mut v = from_id(v, n);
        for k in 2..=n {
            v[..k].reverse();
            let id = to_id(&v);
            que.push_back((d + 1, id));
            v[..k].reverse();
        }
    }
    for s in s {
        let mut v = vec![0; n];
        for i in 0..n {
            v[i] = (s[i] as u8 - b'A') as usize;
        }
        let id = to_id(&v);
        puts!("{}\n", dist[id]);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
