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

fn sort_to(a: i32, b: i32, r: &mut [i64],
           ops: &mut Vec<(i32, i32)>, dec: bool) {
    let to1 = if a == b { b % 3 + 1 } else { 6 - a - b };
    let to2 = 6 - to1 - b;
    assert_ne!(b, to1);
    assert_ne!(b, to2);
    assert_ne!(a, to1);
    if r.len() <= 1 {
        if a != b {
            ops.push((a, b));
        }
        return;
    }
    let mid = r.len() / 2;
    sort_to(a, to1, &mut r[mid..], ops, !dec);
    sort_to(a, to2, &mut r[..mid], ops, !dec);
    // merge
    let mut merged = vec![];
    let mut xpos = 0;
    let mut ypos = mid;
    r[..mid].reverse();
    r[mid..].reverse();
     while xpos < mid && ypos < r.len() {
        if (r[xpos] < r[ypos]) ^ dec {
            merged.push(r[xpos]);
            xpos += 1;
            ops.push((to2, b));
        } else {
            merged.push(r[ypos]);
            ypos += 1;
            ops.push((to1, b));
        }
    }
    while xpos < mid {
        merged.push(r[xpos]);
        xpos += 1;
        ops.push((to2, b));
    }
    while ypos < r.len() {
        merged.push(r[ypos]);
        ypos += 1;
        ops.push((to1, b));
    }
    for i in 0..r.len() {
        r[i] = merged[i];
    }
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input!(n: usize, r: [i64; n]);
    let mut ops = vec![];
    let mut r = r;
    let mut board = vec![r.clone(), vec![], vec![]];
    sort_to(1, 1, &mut r, &mut ops, true);
    puts!("{}\n", ops.len());
    for &(a, b) in &ops {
        puts!("{} {}\n", a, b);
        let x = board[a as usize - 1].pop().unwrap();
        board[b as usize - 1].push(x);
    }
    for i in 0..n {
        assert_eq!(board[0][i], (n - i) as i64);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
