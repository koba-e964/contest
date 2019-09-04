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

fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize, x: i64,
        a: [i64; n],
    }
    let mut a = a;
    let mut sum = x;
    let mut vacant = vec![];
    for i in 0..n {
        if a[i] != -1 {
            sum ^= a[i];
        } else {
            vacant.push(i);
        }
    }
    match vacant.len() {
        0 => if sum != 0 {
            puts!("-1\n");
            return;
        },
        1 => if sum > x {
            puts!("-1\n");
            return;
        } else {
            a[vacant[0]] = sum;
        },
        _ => {
            if sum > x && (sum ^ x) > x {
                puts!("-1\n");
                return;
            }
            for &v in &vacant {
                a[v] = 0;
            }
            if sum > x {
                a[vacant[0]] = sum ^ x;
                a[vacant[1]] = x;
            } else {
                a[vacant[0]] = sum;
            }
        },
    }
    for i in 0..n {
        puts!("{}{}", a[i], if i + 1 == n { "\n" } else { " " });
    }
}
