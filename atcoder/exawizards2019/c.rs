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

// bias: +1
fn sim(n: usize, s: &[char], mut v: usize, td: &[(char, char)]) -> usize {
    for &(t, d) in td {
        if s[v] == t {
            if d == 'L' {
                if v == 0 {
                    return 0;
                }
                v -= 1;
            } else {
                v += 1;
                if v >= n {
                    return n + 1;
                }
            }
        }
    }
    v + 1
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize, q: usize,
        s: chars,
        td: [(chars, chars); q],
    }
    let td: Vec<(char, char)> = td.into_iter().map(|(a, b)| (a[0], b[0])).collect();
    let mut pass = n + 1;
    let mut fail = 0;
    while pass - fail > 1 {
        let mid = (pass + fail) / 2;
        if sim(n, &s, mid - 1, &td) > 0 {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    let fst = pass;
    pass = 0;
    fail = n + 1;
    while fail - pass > 1 {
        let mid = (pass + fail) / 2;
        if sim(n, &s, mid - 1, &td) < n + 1 {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    let snd = pass;
    puts!("{}\n", if snd >= fst { snd + 1 - fst } else { 0 });
        
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
