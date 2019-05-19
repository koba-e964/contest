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
    input!(m: usize, k: i64);
    macro_rules! fail {
        () => {
            puts!("-1\n");
            return;
        };
    }
    if m == 0 {
        if k != 0 {
            fail!();
        }
        puts!("0 0\n");
        return;
    }
    if m == 1 {
        if k != 0 {
            fail!();
        }
        puts!("0 0 1 1\n");
        return;
    }
    if k >= 1 << m {
        fail!();
    }
    let mut ans = vec![];

    if k == 0 {
        for i in 0..1 << m {
            ans.push(i);
            ans.push(i);
        }
    } else {
        let vacant = k & -k;
        ans.push(0);
        ans.push(k);
        for i in 0..1 << m {
            if (i & vacant) != 0 { continue; }
            if i == 0 { continue; }
            ans.push(i);
            ans.push(i ^ k);
        }
        ans.push(k);
        ans.push(0);
        for i in 0..1 << m {
            if (i & vacant) != 0 { continue; }
            if i == 0 { continue; }
            ans.push(i ^ k);
            ans.push(i);
        }
    }

    for i in 0..ans.len() {
        puts!("{}{}", ans[i], if i + 1 == ans.len() { "\n" } else { " " });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
