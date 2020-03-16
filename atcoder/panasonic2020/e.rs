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

fn mat(a: &[char], b: &[char]) -> (Vec<bool>, Vec<bool>) {
    let n = a.len();
    let m = b.len();
    let mut fst = vec![false; n + 1];
    let mut snd = vec![false; m + 1];
    for i in 0..n + 1 {
        let mut ok = true;
        for j in 0..m {
            if i + j < n && a[i + j] != b[j] && a[i + j] != '?' && b[j] != '?' {
                ok = false;
                break;
            }
        }
        fst[i] = ok;
    }
    for i in 0..m + 1 {
        let mut ok = true;
        for j in 0..n {
            if i + j < m && b[i + j] != a[j] && b[i + j] != '?' && a[j] != '?' {
                ok = false;
                break;
            }
        }
        snd[i] = ok;
    }
    (fst, snd)
}

fn calc(a: usize, b: usize, c: usize, ab: &[bool], ac: &[bool],
        bc: &[bool], cb: &[bool]) -> usize {
    let mut mi = 1 << 20;
    for i in 0..a + c + 1 {
        if i < a && !ab[i] { continue; }
        for j in 0..a + b + 1 {
            if j < a && !ac[j] {
                continue;
            }
            if i <= j && j <= i + b && !bc[j - i] {
                continue;
            }
            if i >= j && i <= j + c && !cb[i - j] {
                continue;
            }
            mi = min(mi, max(a, max(b + i, c + j)));
        }
    }
    mi
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        a: chars,
        b: chars,
        c: chars,
    }
    let (ab, ba) = mat(&a, &b);
    let (ac, ca) = mat(&a, &c);
    let (bc, cb) = mat(&b, &c);
    let mut ma = 1 << 20;
    ma = min(ma, calc(a.len(), b.len(), c.len(), &ab, &ac, &bc, &cb));
    ma = min(ma, calc(b.len(), a.len(), c.len(), &ba, &bc, &ac, &ca));
    ma = min(ma, calc(c.len(), b.len(), a.len(), &cb, &ca, &ba, &ab));
    puts!("{}\n", ma);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
