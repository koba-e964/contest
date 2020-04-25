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
        n: usize, k: usize,
        sc: [(chars, i64); n],
    }
    const INF: i64 = 1 << 50;
    let mut dp = vec![INF; 3 * k + 1];
    let mut pre = vec![vec![0; n]; 3];
    for i in 0..n {
        let (ref s, _c) = sc[i];
        for j in 0..3 {
            let ch = ['J', 'O', 'I'][j];
            pre[j][i] = s.iter().filter(|&&cc| cc == ch).count();
        }
    }
    let mut target = vec!['+'; 3 * k];
    for i in 0..3 {
        let c = ['J', 'O', 'I'][i];
        for j in 0..k {
            target[i * k + j] = c;
        }
    }
    dp[0] = 0;
    for i in 0..3 * k {
        let ph = i / k;
        let idx = i % k;
        for j in 0..n {
            let (ref s, c) = sc[j]; 
            if idx + pre[ph][j] < k {
                dp[i + pre[ph][j]] = min(dp[i + pre[ph][j]], dp[i] + c);
            } else {
                // full scan
                let mut pos = i;
                for &ch in s {
                    if pos < 3 * k && ch == target[pos] {
                        pos += 1;
                    }
                }
                dp[pos] = min(dp[pos], dp[i] + c);
            }
        }
    }
    puts!("{}\n", if dp[3 * k] >= INF { -1 } else { dp[3 * k] });
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
