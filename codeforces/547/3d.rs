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

fn get_freq(a: &[char]) -> Vec<Vec<usize>> {
    let mut ret = vec![vec![]; 27];
    for i in 0..a.len() {
        let a = a[i];
        if a == '?' {
            ret[26].push(i);
        } else {
            ret[(a as u8 - b'a') as usize].push(i);
        }
    }
    ret
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize,
        l: chars,
        r: chars,
    }
    let mut fl = get_freq(&l);
    let mut fr = get_freq(&r);
    let mut matching = vec![];
    for i in 0..26 {
        while !fl[i].is_empty() && !fr[i].is_empty() {
            let x = fl[i].pop().unwrap();
            let y = fr[i].pop().unwrap();
            matching.push((x, y));
        }
    }
    // match ?
    for i in 0..26 {
        while !fl[i].is_empty() && !fr[26].is_empty() {
            let x = fl[i].pop().unwrap();
            let y = fr[26].pop().unwrap();
            matching.push((x, y));
        }
        while !fl[26].is_empty() && !fr[i].is_empty() {
            let x = fl[26].pop().unwrap();
            let y = fr[i].pop().unwrap();
            matching.push((x, y));
        }
    }
    while !fl[26].is_empty() && !fr[26].is_empty() {
        let x = fl[26].pop().unwrap();
        let y = fr[26].pop().unwrap();
        matching.push((x, y));
    }
    puts!("{}\n", matching.len());
    for &(x, y) in &matching {
        puts!("{} {}\n", x + 1, y + 1);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
