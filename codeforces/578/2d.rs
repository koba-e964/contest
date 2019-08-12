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
    input! {
        n: usize, k: usize,
        a: [chars; n],
    }
    let mut row = vec![vec![0i32; n - k + 1]; n];
    let mut orig_row = vec![0i32; n];
    for i in 0..n {
        let whole = (0..n).filter(|&j| a[i][j] == 'B').count();
        let mut bk = (0..k).filter(|&j| a[i][j] == 'B').count();
        orig_row[i] = if whole == 0 { 1 } else { 0 };
        for j in k..n + 1 {
            row[i][j - k] = if whole != 0 && bk == whole { 1 } else { 0 };
            if j < n {
                if a[i][j] == 'B' {
                    bk += 1;
                }
                if a[i][j - k] == 'B' {
                    bk -= 1;
                }
            }
        }
    }
    let mut col = vec![vec![0i32; n]; n - k + 1];
    let mut orig_col = vec![0i32; n];
    for i in 0..n {
        let whole = (0..n).filter(|&j| a[j][i] == 'B').count();
        let mut bk = (0..k).filter(|&j| a[j][i] == 'B').count();
        orig_col[i] = if whole == 0 { 1 } else { 0 };
        for j in k..n + 1 {
            col[j - k][i] = if whole != 0 && bk == whole { 1 } else { 0 };
            if j < n {
                if a[j][i] == 'B' {
                    bk += 1;
                }
                if a[j - k][i] == 'B' {
                    bk -= 1;
                }
            }
        }
    }
    let mut rowsum = vec![vec![0; n - k + 1]; n - k + 1];
    for j in 0..n - k + 1 {
        let mut sum = 0;
        for i in 0..k {
            sum += row[i][j];
        }
        for i in k..n + 1 {
            rowsum[i - k][j] = sum;
            if i < n {
                sum += row[i][j];
                sum -= row[i - k][j];
            }
        }
    }
    let mut colsum = vec![vec![0; n - k + 1]; n - k + 1];
    for i in 0..n - k + 1 {
        let mut sum = 0;
        for j in 0..k {
            sum += col[i][j];
        }
        for j in k..n + 1 {
            colsum[i][j - k] = sum;
            if j < n {
                sum += col[i][j];
                sum -= col[i][j - k];
            }
        }
    }
    let mut ma = 0;
    for i in 0..n - k + 1 {
        for j in 0..n - k + 1 {
            ma = max(ma, rowsum[i][j] + colsum[i][j]);
        }
    }
    for i in 0..n {
        ma += orig_row[i] + orig_col[i];
    }
    puts!("{}\n", ma);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
