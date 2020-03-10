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

fn oddness_vec(n: usize, m: usize, aa: &[Vec<i32>]) -> i64 {
    let mut a = 0;
    let mut acc = vec![vec![0; m + 1]; n + 1];
    for i in 0..n {
        for j in 0..m {
            acc[i + 1][j + 1] = acc[i][j + 1] ^ acc[i + 1][j] ^ acc[i][j]
                ^ aa[i][j];
        }
    }
    for i in 1..n + 1 {
        for j in 0..i {
            for k in 1..m + 1 {
                for l in 0..k {
                    if (acc[i][k] ^ acc[i][l] ^ acc[j][k] ^ acc[j][l]) == 1 {
                        a += 1;
                    }
                }
            }
        }
    }
    a
}

fn oddness(n: usize, m: usize, pat: i64) -> i64 {
    let mut a = 0;
    let mut acc = vec![vec![0; m + 1]; n + 1];
    for i in 0..n {
        for j in 0..m {
            let pc = (pat >> (i * m + j)) & 1;
            acc[i + 1][j + 1] = acc[i][j + 1] ^ acc[i + 1][j] ^ acc[i][j] ^ pc;
        }
    }
    for i in 1..n + 1 {
        for j in 0..i {
            for k in 1..m + 1 {
                for l in 0..k {
                    if (acc[i][k] ^ acc[i][l] ^ acc[j][k] ^ acc[j][l]) == 1 {
                        a += 1;
                    }
                }
            }
        }
    }
    a
}

fn flip(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = a.len();
    let m = a[0].len();
    let mut ret = vec![vec![0; n]; m];
    for i in 0..n {
        for j in 0..m {
            ret[j][i] = a[i][j];
        }
    }
    ret
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize, m: usize,
    }
    let flip = n > m;
    let (n, m) = if flip {
        (m, n)
    } else {
        (n, m)
    };
    let mut ans = vec![vec![0; 1 << m]; 1 << n];
    for i in 0usize..1 << n {
        for j in 0usize..1 << m {
            let pc = (i & j).count_ones();
            ans[i][j] = (pc % 2) as i32;
        }
    }
    for i in 0..1 << n {
        for j in (0..(1 << m) - 1).rev() {
            ans[i][j + 1] ^= ans[i][j];
        }
    }
    for i in (0..(1 << n) - 1).rev() {
        for j in 0..1 << m {
            ans[i + 1][j] ^= ans[i][j];
        }
    }
    let mut t = vec![vec![0; (1 << m) - 1]; (1 << n) - 1];
    for i in 0..(1 << n) - 1 {
        for j in 0..(1 << m) - 1 {
            t[i][j] = ans[i + 1][j + 1];
        }
    }
    let (n, m) = if flip {
        (m, n)
    } else {
        (n, m)
    };
    if flip {
        t = ::flip(t);
    }
    for i in 0..(1 << n) - 1 {
        for j in 0..(1 << m) - 1 {
            puts!("{}", t[i][j]);
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
