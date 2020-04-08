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

#[allow(unused)]
macro_rules! debug {
    ($($format:tt)*) => (write!(std::io::stderr(), $($format)*).unwrap());
}
#[allow(unused)]
macro_rules! debugln {
    ($($format:tt)*) => (writeln!(std::io::stderr(), $($format)*).unwrap());
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize, k: usize,
        s: chars,
    }
    let mut lft = vec![];
    for i in 0..n {
        if s[i] == 'L' {
            lft.push(i as usize);
        }
    }
    let m = lft.len();
    let mut st = vec![0; m];
    let mut turn = vec![0; m];
    let mut last = vec![0; m];
    let mut ma = 0;
    let mut sum = 0;
    for i in 0..m {
        turn[i] = lft[i] - i;
        last[i] = turn[i];
        if i > 0 {
            last[i] = max(last[i], if last[i - 1] == 0 { 0 } else {
                last[i - 1] + 1
            });
        }
        ma = max(ma, last[i]);
        sum += turn[i];
    }
    if k < ma || k > sum {
        puts!("-1\n");
        return;
    }
    let mut cur = k;
    for i in (0..m).rev() {
        st[i] = cur - turn[i];
        cur = max(cur, if i > 0 { last[i - 1] } else { 0 } + turn[i]) - turn[i];
    }
    let mut ops = vec![vec![]; k];
    for i in 0..m {
        for j in st[i]..st[i] + turn[i] {
            let now = i + turn[i] - (j - st[i]);
            ops[j].push(now);
        }
    }
    if false {
        eprintln!("st = {:?}", st);
        eprintln!("turn = {:?}", turn);
        eprintln!("last = {:?}", last);
        eprintln!("op = {:?}", ops);
    }
    for i in 0..k {
        assert!(!ops[i].is_empty());
        puts!("{}", ops[i].len());
        for &v in &ops[i] {
            puts!(" {}", v);
        }
        puts!("\n");
    }
}

// 4 2 RLLL -> -1
// 5 1 LLLRL -> 1 4

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
