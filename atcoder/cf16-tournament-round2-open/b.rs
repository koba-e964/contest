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

fn cmp(a: &[i64], ac: i64, b: &[i64]) -> Ordering {
    let n = a.len();
    let mut cur = vec![1];
    for i in 1..n {
        let mut tmp = cur[i - 1];
        tmp *= ac + i as i64 - 1;
        tmp /= i as i64;
        cur.push(tmp);
        let mut ax: i64 = 0;
        for j in 0..i + 1 {
            ax = ax.saturating_add(a[j].saturating_mul(cur[i - j]));
        }
        if ax != b[i] {
            return ax.cmp(&b[i]);
        }
    }
    Ordering::Equal
}

fn step(a: &[i64], ac: i64, b: &[i64]) -> Result<i64, ()> {
    let n = a.len();
    if a[0] > b[0] {
        return Err(());
    }
    if a[0] < b[0] {
        return Ok(0);
    }
    if n == 1 {
        return Err(());
    }
    let a1 = a[1] + ac * a[0];
    if a1 < b[1] {
        return Ok(0);
    }
    let x = (a1 - b[1]) / a[0];
    // x times?
    if b[0] * x + b[1] < a1 {
        return Ok(x + 1);
    }
    let c = if ac > x {
        cmp(a, ac - x, b) == Ordering::Less
    } else {
        cmp(b, x - ac, a) == Ordering::Greater
    };
    if c {
        Ok(x)
    } else {
        Ok(x + 1)
    }
}

fn calc(n: usize, a: &[Vec<i64>]) -> Result<i64, ()> {
    let mut tot = 0;
    let mut cur = 0;
    for i in 0..n - 1 {
        cur = step(&a[i], cur, &a[i + 1])?;
        tot += cur;
    }
    Ok(tot)
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize, m: usize,
        a: [[i64; m]; n],
    }
    let ans = calc(n, &a).unwrap_or(-1);
    puts!("{}\n", ans);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
