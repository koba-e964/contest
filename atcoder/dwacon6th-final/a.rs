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

fn trans(a: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let n = a.len();
    let m = a[0].len();
    let mut dp = vec![vec![0; n]; m];
    for i in 0..n {
        for j in 0..m {
            dp[j][i] = a[i][j];
        }
    }
    dp
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    macro_rules! fail {
        () => (puts!("No\n"); return;);
    }
    input! {
        h: usize, w: usize,
    }
    let mut h = h;
    let mut w = w;
    let mut flip = false;
    if h > w {
        std::mem::swap(&mut h, &mut w);
        flip = true;
    }
    let mut tbl = vec![vec![0; w]; h];
    if h == 1 {
        let r = w % 7;
        if r != 2 && r != 5 && r != 0 {
            fail!();
        }
        for i in 0..w / 7 {
            for j in 0..2 {
                tbl[0][7 * i + j] = 2;
            }
            for j in 2..7 {
                tbl[0][7 * i + j] = 5;
            }
        }
        if r == 5 {
            for i in (0..w - 5).rev() {
                tbl[0][i + 5] = tbl[0][i];
            }
            for i in 0..5 {
                tbl[0][i] = 5;
            }
        } else if r == 2 {
            for i in 0..2 {
                tbl[0][w - 2 + i] = 2;
            }
        }
    }
    if h == 2 {
        let r = w % 7;
        if r != 0 && r != 1 && r != 6 {
            fail!();
        }
        for i in 0..2 {
            for j in 0..w {
                tbl[i][j] = 5;
            }
        }
        for i in 0..w / 7 {
            tbl[0][7 * i] = 2;
            tbl[1][7 * i] = 2;
            tbl[0][7 * i + 4] = 2;
            tbl[1][7 * i + 3] = 2;
        }
        if r == 1 {
            tbl[0][w - 1] = 2;
            tbl[1][w - 1] = 2;
        } else if r == 6 {
            for i in 0..2 {
                for j in (0..w - 6).rev() {
                    tbl[i][j + 6] = tbl[i][j];
                }
                for j in 0..6 {
                    tbl[i][j] = 5;
                }
            }
            tbl[0][3] = 2;
            tbl[1][2] = 2;
        }
    }
    if h == 3 {
        if w != 3 {
            fail!();
        }
        tbl[0] = vec![2, 5, 2];
        tbl[1] = vec![2, 5, 2];
        tbl[2] = vec![5, 5, 5];
    }
    if h >= 4 {
        fail!();
    }
    if flip {
        tbl = trans(tbl);
        std::mem::swap(&mut h, &mut w);
    }
    puts!("Yes\n");
    for i in 0..h {
        for j in 0..w {
            puts!("{}", tbl[i][j]);
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
