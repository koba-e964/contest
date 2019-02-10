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
        l: usize,
        a: [i64; l],
    }
    const INF: i64 = 1 << 58;
    // 0: not started
    // 1: started, odd not started
    // 2: odd started
    // 3: started, odd ended
    // 4: ended
    let mut dp = vec![[INF; 5]; l + 1];
    dp[0][0] = 0;
    for i in 0..l {
        let mut cost = vec![0; 3];
        cost[0] = a[i];
        if a[i] == 0 {
            cost[1] = 1;
            cost[2] = 2;
        } else {
            let r = a[i] % 2;
            cost[1] = 1 - r;
            cost[2] = r;
        }
        // 5: not allowed
        let trans =
            [
                [0, 2, 1],
                [4, 2, 1],
                [4, 2, 3],
                [4, 5, 3],
                [4, 5, 5],
            ];
        for j in 0..5 {
            for k in 0..3 {
                let to = trans[j][k];
                if to >= 5 { continue; }
                dp[i + 1][to] = min(dp[i + 1][to], dp[i][j] + cost[k]);
            }
        }
    }
    //eprintln!("dp = {:?}", dp);
    let mi: i64 = dp[l].iter().min().cloned().unwrap();
    puts!("{}\n", mi);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
