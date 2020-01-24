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
    macro_rules! fail {
        () => (puts!("NO\n"); return;);
    }
    input! {
        n: usize, k: usize, p: usize,
        a: [i64; n],
    }
    let p = k - p;
    let mut odd: Vec<i64> = a.iter().cloned().filter(|&x| x % 2 == 1).collect();
    let even: Vec<i64> = a.iter().cloned().filter(|&x| x % 2 == 0).collect();
    let o = odd.len();
    if o < p || (o + p) % 2 != 0 {
        fail!();
    }
    let mut ans = vec![vec![]; k];
    for i in 0..p {
        ans[i].push(odd.pop().unwrap());
    }
    let mut rem = vec![];
    for i in 0..odd.len() / 2 {
        rem.push(Ok((odd[2 * i], odd[2 * i + 1])));
    }
    for i in 0..even.len() {
        rem.push(Err(even[i]));
    }
    if rem.len() < k - p {
        fail!();
    }
    for i in 0..rem.len() {
        let targ = min(i + p, k - 1);
        match rem[i] {
            Ok((x, y)) => ans[targ].extend_from_slice(&[x, y]),
            Err(x) => ans[targ].push(x),
        }
    }
    puts!("YES\n");
    for i in 0..k {
        puts!("{}", ans[i].len());
        for j in 0..ans[i].len() {
            puts!(" {}", ans[i][j]);
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
