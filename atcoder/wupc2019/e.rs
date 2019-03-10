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

// Manacher http://snuke.hatenablog.com/entry/2014/12/02/235837
fn manacher<T: PartialEq>(tmp: &[T]) -> Vec<usize> {
    let n = tmp.len();
    let mut r = vec![0; n];
    {
        let mut i = 0;
        let mut j = 0;
        while i < n {
            while i >= j && i + j < n && tmp[i - j] == tmp[i + j] {
                j += 1;
            }
            r[i] = j;
            let mut k = 1;
            while i >= k && i + k < n && k + r[i - k] < j {
                r[i + k] = r[i - k];
                k += 1;
            }
            i += k;
            j -= k;
        }
    }
    r
}

fn calc(a: &[usize]) -> i64 {
    let n = a.len();
    let mut tmp = vec![0; 2 * n - 1];
    for i in 0..n {
        tmp[2 * i] = a[i] + 1;
    }
    let r = manacher(&tmp);
    let mut cnt = 0;
    for i in 0..n - 1 {
        if r[i] >= i + 1 && r[n + i] >= n - i - 1 {
            cnt += 1;
        }
    }
    cnt
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        m: usize,
        n: usize,
        a: [chars; m],
    }
    let mut row = vec![0; m];
    let mut col = vec![0; n];
    for i in 0..m {
        for j in 0..n {
            if a[i][j] == '1' {
                row[i] += 1;
                col[j] += 1;
            }
        }
    }
    puts!("{}\n", calc(&row) * calc(&col));
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
