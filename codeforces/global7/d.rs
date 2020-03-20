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

// Manacher http://snuke.hatenablog.com/entry/2014/12/02/235837
// Verified by https://atcoder.jp/contests/wupc2019/submissions/4540033
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

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input!(s: [chars]);
    for s in s {
        let mut ss = vec![];
        let n = s.len();
        for i in 0..n {
            ss.push(s[i]);
            ss.push('+');
        }
        let pal = manacher(&ss);
        assert_eq!(pal.len(), 2 * n);
        let mut com = 0;
        for i in 0..n / 2 {
            if s[i] == s[n - 1 - i] {
                com += 1;
            } else {
                break;
            }
        }
        let mut ma = (2 * com, (com, com));
        // s[0, com - 1] = reverse(s[n - com, n - 1])
        for i in 2 * com..2 * (n - com) {
            let r = pal[i];
            let mut lo = (i + 2 - r) / 2;
            let mut hi = (i + r - 1) / 2;
            // [lo, hi]
            if lo < com {
                let k = com - lo;
                lo += k;
                hi -= k;
            }
            if hi >= n - com {
                let k = hi - (n - com) + 1;
                lo += k;
                hi -= k;
            }
            if lo <= hi {
                if lo == com {
                    ma = max(ma, (com + hi + 1, (hi + 1, com)));
                } else if hi == n - com - 1 {
                    ma = max(ma, (com + n - lo, (com, n - lo)));
                }
            }
        }
        let (_len, (p, q)) = ma;
        for i in 0..p {
            puts!("{}", s[i]);
        }
        for i in n - q..n {
            puts!("{}", s[i]);
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
