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

fn reach(n: usize, s: &[char], a: usize) -> Vec<bool> {
    let mut ret = vec![false; n];
    ret[a] = true;
    let mut pos = a;
    while pos < n {
        if s[pos] != '#' {
            ret[pos] = true;
        }
        if pos + 1 < n && s[pos + 1] == '.' {
            pos += 1;
            continue;
        }
        if pos + 2 < n && s[pos + 2] == '.' {
            pos += 1;
            continue;
        }
        break;
    }
    ret
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    macro_rules! fail {
        () => {
            puts!("No\n");
            return;
        }
    }
    input! {
        n: usize, a: usize1, b: usize1, c: usize1, d: usize1,
        s: chars,
    }
    let ra = reach(n, &s, a);
    let rb = reach(n, &s, b);
    //eprintln!("ra = {:?}, rb = {:?}", ra, rb);
    if !ra[c] || !rb[d] {
        fail!();
    }
    let swap = c > d;
    if swap {
        // check for space with width >= 3
        let mut found = false;
        for i in 0..min(n - 3, d - 1) + 1 {
            if ra[i] && rb[i + 1] && rb[i + 2] {
                found = true;
                break;
            }
        }
        if !found {
            fail!();
        }
    }
    puts!("Yes\n");
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
