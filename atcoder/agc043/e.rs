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

fn conn<T: Eq + Copy + std::fmt::Debug>(ans: &mut Vec<T>, ops: &[T]) {
    assert_eq!(ans.last(), ops.first());
    for i in 1..ops.len() {
        ans.push(ops[i]);
    }
}

fn calc(n: usize, bits: usize) -> Vec<(i64, i64)> {
    if bits == 0 {
        return vec![(0, 0)];
    }
    let mut ma = 0;
    for i in 0..n {
        if (bits & 1 << i) != 0 {
            ma = i;
        }
    }
    let mut sub = calc(n, bits ^ 1 << ma);
    let mut t = vec![];
    for i in 0..ma + 1 {
        t.push((i as i64, 0));
    }
    t.push((ma as i64, 1));
    t.push((ma as i64 + 1, 1));
    for i in (0..ma + 2).rev() {
        t.push((i as i64, 0));
    }
    if bits == 1 << ma {
        return t;
    }
    let mut ans = vec![(0, 0)];
    // commutator
    conn(&mut ans, &sub);
    conn(&mut ans, &t);
    sub.reverse();
    t.reverse();
    conn(&mut ans, &sub);
    conn(&mut ans, &t);
    ans
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize,
        a: chars,
    }
    for i in 0..1 << n {
        for j in 0..1 << n {
            if (i & j) == i && a[i] == '0' && a[j] == '1' {
                puts!("Impossible\n");
                return;
            }
        }
    }
    if a[0] == '0' {
        puts!("Impossible\n");
        return;
    }
    let mut ans = vec![(0, 0)];
    for i in 1..1 << n {
        if a[i] == '1' {
            continue;
        }
        conn(&mut ans, &calc(n, i));
    }
    assert!(ans.len() - 1 <= 250_000);
    puts!("Possible\n");
    puts!("{}\n", ans.len() - 1);
    for (x, y) in ans {
        puts!("{} {}\n", x, y);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
