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
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        n: usize,
        ab: [[i64; 2 * n]; 2],
    }
    let mut ans = vec!['+'; 2 * n];
    // B
    let mut mi = vec![[2 * n; 2]; 2 * n + 1];
    let mut ma = vec![[0; 2]; 2 * n + 1];
    mi[0] = [0, 0];
    for i in 0..2 * n {
        for j in 0..2 {
            if mi[i][j] > ma[i][j] {
                continue;
            }
            let pre = if i > 0 { ab[j][i - 1] } else { -1 << 28 };
            for k in 0..2 {
                if pre <= ab[k][i] {
                    mi[i + 1][k] = min(mi[i + 1][k], mi[i][j] + k);
                    ma[i + 1][k] = max(ma[i + 1][k], ma[i][j] + k);
                }
            }
        }
    }
    let mut cur = 2;
    for i in 0..2 {
        if mi[2 * n][i] <= n && n <= ma[2 * n][i] {
            cur = i;
            break;
        }
    }
    if cur >= 2 {
        puts!("-1\n");
        return;
    }
    let mut rem_b = n;
    for i in (0..2 * n).rev() {
        let k = cur;
        ans[i] = ['A', 'B'][k];
        rem_b -= k;
        let mut pre = 2;
        for j in 0..2 {
            if mi[i][j] > ma[i][j] {
                continue;
            }
            let pre_elem = if i > 0 { ab[j][i - 1] } else { -1 << 28 };
            if pre_elem <= ab[k][i] {
                if mi[i][j] <= rem_b && rem_b <= ma[i][j] {
                    pre = j;
                    break;
                }
            }
        }
        assert_ne!(pre, 2);
        cur = pre;
    }
    for i in 0..2 * n {
        puts!("{}", ans[i]);
    }
    puts!("\n");
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
