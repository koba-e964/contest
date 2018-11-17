#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Write, BufWriter};
// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
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

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    input! {
        n: usize,
        k: usize,
        x: usize,
        a: [i64; n],
    }
    const INF: i64 = 1 << 60;
    let mut dp = vec![vec![-INF; n + 1]; 2];
    dp[0][0] = 0;
    for _ in 0 .. x {
        dp[1].splice(.., (0 .. n + 1).map(|_| 0));
        // Manage candidates of max values in [i - k, i)
        let mut st = VecDeque::new();
        st.push_back((0, dp[0][0]));
        for i in 1 .. n + 1 {
            dp[1][i] = st.front().unwrap().1 + a[i - 1];
            // Update
            while let Some((idx, val)) = st.pop_back() {
                if val > dp[0][i] {
                    st.push_back((idx, val));
                    break;
                }
            }
            st.push_back((i, dp[0][i]));
            if i >= k {
                if let Some((idx, val)) = st.pop_front() {
                    if idx != i - k {
                        st.push_front((idx, val));
                    }
                }
            }
        }
        dp.swap(0, 1);
    }
    let mut ma = -1;
    for i in n + 1 - k .. n + 1 { ma = max(ma, dp[0][i]); }
    puts!("{}\n", ma);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
