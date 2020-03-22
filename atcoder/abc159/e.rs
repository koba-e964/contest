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
        h: usize, w: usize, k: usize,
        s: [chars; h],
    }
    let mut mi = h + w;
    for bits in 0usize..1 << (h - 1) {
        let bc = bits.count_ones() as usize;
        let mut blk = vec![0; bc + 1];
        let mut to = vec![0; h];
        let mut me = 0;
        for i in 0..h {
            to[i] = me;
            if i + 1 < h && (bits & 1 << i) != 0 {
                me += 1;
            }
        }
        let mut dp = vec![vec![0; w]; bc + 1];
        for i in 0..w {
            for j in 0..h {
                dp[to[j]][i] += if s[j][i] == '1' { 1 } else { 0 };
            }
        }
        let mut ok = true;
        for i in 0..bc + 1 {
            for j in 0..w {
                if dp[i][j] > k {
                    ok = false;
                }
            }
        }
        if !ok { continue; }
        let mut numcut = 0;
        for i in 0..w {
            let mut needcut = false;
            for j in 0..bc + 1 {
                if blk[j] + dp[j][i] > k {
                    needcut = true;
                }
            }
            if needcut {
                numcut += 1;
                for j in 0..bc + 1 { blk[j] = 0; }
            }
            for j in 0..bc + 1 {
                blk[j] += dp[j][i];
            }
        }
        mi = min(mi, numcut + bc);
    }
    puts!("{}\n", mi);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
