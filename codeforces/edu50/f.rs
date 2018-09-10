#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
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
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

const B1: i32 = 5;
const B2: i32 = 40;

// returns #{x | exists y. x = y^v, 2 <= x <= n}.
fn calc(n: i64, v: i32, tbl: &[Vec<i64>]) -> i64 {
    if v == 1 {
        return n - 1;
    }
    if v >= B2 {
        return if n >= 1 << v { 1 } else { 0 };
    }
    if v >= B1 {
        return match tbl[v as usize].binary_search(&n) {
            Ok(i) => i as i64,
            Err(i) => (i - 1) as i64,
        } - 1;
    }
    let mut fail: i64 = 1 << ((60 + v) / v);
    let mut pass = 0;
    while fail - pass > 1 {
        let mid = (pass + fail) / 2;
        let mut tmp = mid;
        for _ in 0 .. v - 1 {
            tmp = tmp.saturating_mul(mid);
            if tmp > n { break; }
        }
        if tmp <= n { pass = mid; }
        else { fail = mid; }
    }
    pass - 1
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    input!{
        t: usize,
        n: [i64; t],
    }
    let mut coef = [0; 60];
    for i in 1 .. 60 {
        let mut v = i;
        let mut ans = 1;
        let mut p = 2;
        while v > 1 {
            let mut e = 0;
            while v % p == 0 {
                v /= p;
                e += 1;
            }
            if e >= 2 {
                ans = 0;
            }
            if e == 1 {
                ans = -ans;
            }
            p += 1;
        }
        coef[i] = ans;
    }
    let mut tbl = vec![Vec::<i64>::new(); B2 as usize];
    for i in B1 .. B2 {
        let i = i as usize;
        let mut cnt = 0;
        loop {
            let mut tmp: i64 = 1;
            for _ in 0 .. i { tmp = tmp.saturating_mul(cnt); }
            tbl[i].push(tmp);
            if tmp >= 1 << 60 { break; }
            cnt += 1;
        }
    }
    // eprintln!("coef = {:?}", &coef[0..10]);
    for n in n {
        let mut ans = 0;
        for i in 1 .. 60 {
            if coef[i] != 0 {
                let freq = calc(n, i as i32, &tbl);
                //eprintln!("calc({}, {}) = {}", n, i, freq);
                ans += freq * coef[i];
            }
        }
        puts!("{}\n", ans);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
