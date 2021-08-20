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

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
    }
    let mut ev = vec![];
    for i in 0..n {
        ev.push((a[i], i));
        ev.push((b[i], n + i));
    }
    ev.sort();
    let mut del = 0;
    let mut ops = vec![];
    let mut cur = vec![];
    for i in 0..2 * n {
        let (_, y) = ev[i];
        let old = del;
        if y >= n {
            del -= 1;
        } else {
            del += 1;
            cur.push(y);
        }
        if del == 0 {
            if old > 0 {
                cur.reverse();
                for c in cur.drain(..) {
                    for _ in a[c]..b[c] {
                        ops.push((c, 0));
                    }
                }
            } else {
                for c in cur.drain(..) {
                    for _ in b[c]..a[c] {
                        ops.push((c, 1));
                    }
                }
            }
        }
    }
    puts!("{}\n", ops.len());
    for (x, y) in ops {
        puts!("{} {}\n", x + 1, if y == 0 { 'R' } else { 'L' });
    }
}
