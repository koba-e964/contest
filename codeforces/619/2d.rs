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

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: usize, m: usize, k: i64,
    }
    let lim = 4 * n * m - 2 * n - 2 * m;
    if k > lim as i64 {
        puts!("NO\n");
        return;
    }
    puts!("YES\n");
    let mut ans = vec![];
    // row 0
    ans.push((m - 1, "R"));
    ans.push((m - 1, "L"));
    // row i
    for _ in 1..n {
        ans.push((1, "D"));
        ans.push((m - 1, "RUD"));
        ans.push((m - 1, "L"));
    }
    ans.push((n - 1, "U"));
    // trim
    let mut out = vec![];
    let mut rem = k as usize;
    for i in 0..ans.len() {
        let m = ans[i].0;
        let l = ans[i].1.len();
        if rem >= m * l {
            if m > 0 {
                out.push(ans[i]);
            }
            rem -= m * l;
        } else if rem > 0 {
            let q = rem / l;
            if q > 0 {
                out.push((q, ans[i].1));
            }
            rem -= q * l;
            if rem > 0 {
                out.push((1, &ans[i].1[..rem]));
                rem = 0;
            }
            break;
        } else {
            break;
        }
    }
    assert_eq!(rem, 0);
    assert!(out.len() <= 3000);
    puts!("{}\n", out.len());
    for i in 0..out.len() {
        puts!("{} {}\n", out[i].0, out[i].1);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
