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
            bytes.by_ref().map(|r|r.unwrap() as char)
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
    ($next:expr, [graph1; $len:expr]) => {{
        let mut g = vec![vec![]; $len];
        let ab = read_value!($next, [(usize1, usize1)]);
        for (a, b) in ab {
            g[a].push(b);
            g[b].push(a);
        }
        g
    }};
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, [ $t:tt ]) => {{
        let len = read_value!($next, usize);
        read_value!($next, [$t; len])
    }};
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
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
    #[allow(unused)]
    macro_rules! putvec {
        ($v:expr) => {
            for i in 0..$v.len() {
                puts!("{}{}", $v[i], if i + 1 == $v.len() {"\n"} else {" "});
            }
        }
    }
    let n = 18;
    let m = 6;
    input! {
        s: [chars; n],
    }
    let mut r0 = vec![0; n + 1];
    let mut r1 = vec![0; n + 1];
    for i in 0..n {
        for j in 0..m {
            match s[i][j] {
                '0' => r0[i] |= 1 << j,
                '1' => r1[i] |= 1 << j,
                _ => {}
            }
        }
    }
    r0[n] = (1 << m) - 1;
    let mut dp = vec![vec![0u64; 1 << (2 * m)]; n + 1];
    for i in 0..1 << m {
        if (i & r0[0]) == 0 && (i & r1[0]) == r1[0] {
            dp[0][i] = 1;
        }
    }
    for i in 0..n {
        for bits in 0..1usize << (3 * m) {
            let lo = bits >> m;
            let hi = bits & ((1 << (2 * m)) - 1);
            let me = lo & ((1 << m) - 1);
            let nxt = hi & ((1 << m) - 1);
            if (nxt & r0[i + 1]) != 0 || (nxt & r1[i + 1]) != r1[i + 1] {
                continue;
            }
            let mut ok = true;
            for i in 0..m {
                let mut bc = ((me << 1) & (7 << i)).count_ones();
                if (lo & 1 << (m + i)) != 0 {
                    bc += 1;
                }
                if (hi & 1 << i) != 0 {
                    bc += 1;
                }
                let now = (me & 1 << i) != 0;
                if now != (bc >= 3) {
                    ok = false;
                    break;
                }
            }
            if ok {
                dp[i + 1][hi] += dp[i][lo];
            }
        }
    }
    if false {
        for i in 0..n + 1 {
            for j in 0..1 << (2 * m) {
                if dp[i][j] > 0 {
                    eprintln!("{} {} {}", i, j, dp[i][j]);
                }
            }
        }
    }
    let mut tot = 0;
    for i in 0..1 << m {
        tot += dp[n][i << m];
    }
    puts!("{}\n", tot);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
