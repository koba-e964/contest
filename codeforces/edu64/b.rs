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
        s: [chars],
    }
    let chars: Vec<_> = (0..26).map(|i| (b'a' + i) as char).collect();
    for s in &s {
        let mut s: Vec<_> =
            s.iter().map(|&c| (c as u8 - b'a') as usize).collect();
        s.sort();
        let mut freq = vec![0; 26];
        for &c in &s {
            freq[c] += 1;
        }
        s.dedup();
        if s.len() == 1 {
            for _ in 0..freq[s[0]] { puts!("{}", chars[s[0]]); }
            puts!("\n");
            continue;
        }
        if s.len() == 2 {
            if s[1] - s[0] == 1 {
                puts!("No answer\n");
                continue;
            }
            for i in 0..2 {
                for _ in 0..freq[s[i]] { puts!("{}", chars[s[i]]); }
            }
            puts!("\n");
            continue;
        }
        if s.len() == 3 {
            if s[1] - s[0] == 1 && s[2] - s[1] == 1 {
                puts!("No answer\n");
                continue;
            }
            if s[2] - s[1] == 1 {
                s.swap(0, 2);
            }
            for &i in &[1, 2, 0] {
                for _ in 0..freq[s[i]] { puts!("{}", chars[s[i]]); }
            }
            puts!("\n");
            continue;
        }
        let mut ord = vec![];
        for i in 0..s.len() {
            if i % 2 != 0 { ord.push(i); }
        }
        for i in 0..s.len() {
            if i % 2 == 0 { ord.push(i); }
        }
        for i in ord {
            for _ in 0..freq[s[i]] { puts!("{}", chars[s[i]]); }
        }
        puts!("\n");
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
