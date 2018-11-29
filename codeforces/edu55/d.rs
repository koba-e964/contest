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
    macro_rules! fail {
        () => ({
            puts!("NO\n");
            return;
        });
    }
    input! {
        n: usize,
        a: [usize; n],
    }
    let sum: usize = a.iter().sum();
    if sum < 2 * n - 2 {
        fail!();
    }
    let mut ones: Vec<usize> = (0 .. n).filter(|&x| a[x] == 1).collect();
    while ones.len() < 2 {
        for i in 0 .. n {
            if ones.len() == 0 || i != ones[0] {
                ones.push(i);
                break;
            }
        }
    }
    let diam = n - (ones.len() - 2) - 1;
    puts!("YES {}\n", diam);
    let mut mult = vec![true; n];
    for &o in ones.iter() {
        mult[o] = false;
    }
    let mut tak = Vec::new();
    for i in 0 .. n {
        if mult[i] {
            tak.push(i);
        }
    }
    let mut edges = Vec::new();
    assert!(tak.len() >= 1);
    for i in 0 .. tak.len() - 1 {
        edges.push((tak[i], tak[i + 1]));
    }
    let mut deg = a.clone();
    let mut pos = 0;
    edges.push((ones[0], tak[0]));
    edges.push((ones[1], tak[tak.len() - 1]));
    for i in 2 .. ones.len() {
        while deg[tak[pos]] <= 2 { pos += 1; }
        edges.push((tak[pos], ones[i]));
        deg[tak[pos]] -= 1;
    }
    assert_eq!(edges.len(), n - 1);
    puts!("{}\n", edges.len());
    for (u, v) in edges {
        puts!("{} {}\n", u + 1, v + 1);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
