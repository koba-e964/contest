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

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        s: chars,
    }
    let mut acc = vec![0.0; n + 1];
    for i in 0 .. n {
        acc[i + 1] = acc[i] + 1.0 / (i + 2) as f64;
    }
    let mut lft = vec![0; n];
    let mut rgt = vec![0; n];
    let mut cur = 0;
    let mut sum = 0.0;
    for i in 0 .. n {
        if s[i] == '-' {
            cur = 0;
            sum += 1.0;
        } else {
            cur += 1;
            sum += 1.0 / (cur + 1) as f64;
        }
        lft[i] = cur;
    }
    cur = 0;
    for i in (0 .. n).rev() {
        if s[i] == '-' {
            cur = 0;
        } else {
            cur += 1;
        }
        rgt[i] = cur;
    }
    let mut mi = 1.0e20;
    for i in 0 .. n {
        if s[i] == '>' { continue }
        let tmp;
        if i == 0 || i == n - 1 {
            tmp = acc[1] - 1.0;
        } else {
            tmp = match (lft[i - 1], rgt[i + 1]) {
                (0, 0) => acc[1] - 1.0,
                (0, b) => acc[b + 1] - acc[b] - 1.0,
                (a, 0) => acc[a + 1] - acc[a] - 1.0,
                (a, b) => acc[a + b + 1] - acc[a] - acc[b] - 1.0,
            }
        }
        if mi > tmp { mi = tmp; }
    }
    puts!("{}\n", mi + sum);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
