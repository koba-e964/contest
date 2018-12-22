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

    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    }};

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

// How many operations are necessary to make a >= b?
// Or how many operations can you tolerate?
fn delta(a: i64, b: i64) -> i64 {
    if a < b {
        let mut a = a;
        let mut cnt = 0;
        while a < b { a *= 4; cnt += 1; }
        return cnt;
    }
    let mut b = b;
    let mut cnt = 0;
    while a >= b { b *= 4; cnt += 1; }
    return - (cnt - 1);
}

// dec
fn calc(a: &[i64]) -> Vec<i64> {
    let n = a.len();
    let mut dec = vec![0; n + 1];
    dec[1] = 0;
    let mut tot = 0;
    let mut block = vec![(0, -1i64 << 40)];
    for i in 1 .. n {
        //eprintln!("a[{}] = {} comes, prev tot = {}, block = {:?}", i, a[i], tot, block);
        let diff = delta(a[i - 1], a[i]);
        //eprintln!("diff = {}", diff);
        if diff < 0 {
            block.push((i, diff));
        } else {
            for _ in 0 .. diff {
                let (x, mut rem) = block.pop().unwrap();
                tot += (i - x) as i64;
                rem += 1;
                if rem < 0 { block.push((x, rem)); }
            }
        }
        dec[i + 1] = tot;
    }
    dec
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    input! {
        n: usize,
        a: [i64; n],
    }
    let mut a = a;
    // [0,i)
    let dec = calc(&a);
    a.reverse();
    // [i,n)
    let mut inc = calc(&a);
    inc.reverse();
    /*
    eprintln!("inc = {:?}", inc);
    eprintln!("dec = {:?}", dec);
     */
    let mut mi: i64 = 1 << 62;
    for i in 0 .. n {
        let tmp = i as i64 + 2 * (inc[i] + dec[i]);
        mi = min(mi, tmp);
    }
    puts!("{}\n", mi);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
