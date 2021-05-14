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

fn per(p: usize, a: usize) -> usize {
    let mut x = a;
    let mut c = 1;
    while x != 1 {
        x = x * a % p;
        c += 1;
    }
    c
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let r = a % b;
        a = b; b = r;
    }
    a
}

// Tags: hamiltonian-path-of-grid
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
        p: usize, a: usize, b: usize,
    }
    let pa = per(p, a);
    let pb = per(p, b);
    let ea = (p - 1) / pa;
    let eb = (p - 1) / pb;
    if gcd(ea, eb) != 1 {
        puts!("No\n");
        return;
    }
    let mut ans = vec![0; p];
    ans[0] = 1;
    if ea == 1 || eb == 1 {
        let x = if ea == 1 {
            a
        } else {
            b
        };
        for i in 0..p - 1 {
            ans[i + 1] = ans[i] * x % p;
        }
    } else {
        let mut tbl = vec![vec![0; pa]; ea];
        tbl[0][0] = 1;
        for i in 0..ea {
            if i > 0 {
                tbl[i][0] = tbl[i - 1][0] * b % p;
            }
            for j in 1..pa {
                tbl[i][j] = tbl[i][j - 1] * a % p;
            }
        }
        assert!(ea > 1 && pa > 1);
        if pa % 2 == 0 {
            for i in 0..pa {
                for j in 0..ea {
                    let to = if i % 2 == 0 { j } else { ea - 1 - j };
                    ans[i * ea + j] = tbl[to][i];
                }
            }
            ans[p - 1] = 1;
        } else {
            assert_eq!(ea % 2, 0);
            for i in 0..pa {
                ans[i] = tbl[0][i];
            }
            let mut pos = pa;
            for i in 1..ea {
                for j in 0..pa - 1 {
                    let to = if i % 2 == 0 { 1 + j } else { pa - 1 - j };
                    ans[pos] = tbl[i][to];
                    pos += 1;
                }
            }
            for i in (0..ea).rev() {
                ans[pos] = tbl[i][0];
                pos += 1;
            }
            assert_eq!(pos, p);
        }
    }
    puts!("Yes\n");
    putvec!(ans);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
