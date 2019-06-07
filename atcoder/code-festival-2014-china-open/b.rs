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
    input!(n: [i64]);
    for n in n {
        let n = n - 1;
        if n == 0 {
            puts!("0 0\n");
            continue;
        }
        let mut pass = 0;
        let mut fail: i64 = 3 << 28;
        while fail - pass > 1 {
            let mid = (pass + fail) / 2;
            let x = mid * (mid + 1) * 2 + 1;
            if n >= x {
                pass = mid;
            } else {
                fail = mid;
            }
        }
        let rest = n - pass * (pass + 1) * 2 - 1;
        let (x, y) = if rest == 0 {
            (-pass - 1, 0)
        } else if rest == 4 * pass + 3 {
            (pass + 1, 0)
        } else if rest % 2 == 1 {
            let dx = rest / 2;
            let x = -pass - 1 + dx + 1;
            (x, -(pass + 1 - x.abs()))
        } else {
            let dx = rest / 2 - 1;
            let x = -pass - 1 + dx + 1;
            (x, (pass + 1 - x.abs()))
        };
        puts!("{} {}\n", x, y);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
