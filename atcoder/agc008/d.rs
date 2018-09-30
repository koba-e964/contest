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

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    input!{
        n: usize,
        x: [usize1; n],
    }
    let mut pool = vec![];
    for i in 0 .. n {
        pool.push((x[i], i));
    }
    pool.sort();
    let mut pos = 0;
    let mut a = vec![0; n * n];
    for &(x, _idx) in pool.iter() {
        a[x] = n + 1;
    }
    for &(x, idx) in pool.iter() {
        // pack idx idx's to the front
        for _ in 0 .. idx {
            while pos < a.len() && a[pos] != 0 {
                pos += 1;
            }
            if pos >= x {
                puts!("No\n");
                return;
            }
            a[pos] = idx + 1;
            pos += 1;
        }
        a[x] = idx + 1;
    }
    let mut zero = vec![];
    for i in 0 .. n * n {
        if a[i] == 0 {
            zero.push(i);
        }
    }
    let mut pos = 0;
    for &(_, idx) in pool.iter() {
        let num = n - idx - 1;
        for i in 0 .. num {
            a[zero[pos + i]] = idx + 1;
        }
        pos += num;
    }
    assert!(pos == zero.len());
    // check
    let mut occur = vec![Vec::new(); n + 1];
    for (i, &v) in a.iter().enumerate() {
        occur[v].push(i);
    }
    for i in 0 .. n {
        if occur[i + 1][i] != x[i] {
            puts!("No\n");
            return;
        }
    }
    puts!("Yes\n");
    for i in 0 .. n * n {
        puts!("{}{}", if i == 0 { "" } else { " " }, a[i]);
    }
    puts!("\n");
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
