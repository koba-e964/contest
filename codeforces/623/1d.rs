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
        a: [[i32; n]; n],
    }
    let mut rnd;
    // Poor man's random number generator in Rust
    {
        use std::hash::{Hasher, BuildHasher};
        let a = 0xdead_c0de_0013_3331;
        let b = 2457;
        let hm: HashMap<i32, i32> = HashMap::new();
        let mut hash = hm.hasher().build_hasher();
        hash.write_u32(8128);
        let mut x: u64 = hash.finish();
        let get_nxt = move || {
            x = x.wrapping_mul(a).wrapping_add(b);
            (x ^ x << 10) >> 32
        };
        rnd = get_nxt;
    }
    let mut dist = vec![0; n];
    let mut ep = vec![0; n];
    let mut col = vec![false; n];
    const INF: i32 = (1 << 30) - 1;
    let mut mi = INF;
    // e^{-7} ~= 10^{-3}
    for _ in 0..7 << (k - 1) {
        for i in 0..n {
            col[i] = rnd() % 2 == 1;
        }
        dist[0] = 0;
        for i in 1..n {
            dist[i] = INF;
        }
        let mut even = vec![];
        let mut odd = vec![];
        for i in 0..n {
            if col[i] {
                odd.push(i);
            } else {
                even.push(i);
            }
        }
        for _ in 0..k {
            for i in 0..n {
                ep[i] = INF;
            }
            for i in 0..n {
                let iter = if col[i] {
                    &even
                } else {
                    &odd
                };
                for &j in iter {
                    ep[j] = min(ep[j], dist[i] + a[i][j]);
                }
            }
            for i in 0..n {
                dist[i] = ep[i];
            }
        }
        mi = min(mi, dist[0]);
    }
    puts!("{}\n", mi);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
