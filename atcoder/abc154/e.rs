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
        ($($format:tt)*) => (write!(out,$($format)*).unwrap());
    }
    input! {
        n: chars,
        k: usize,
    }
    let n: Vec<_> = n.into_iter().map(|c| (c as u8 - b'0') as usize).collect();
    let m = n.len();
    let r = m - 1;
    let mut tot: usize = 0;
    tot += match k {
        1 => r * 9,
        2 => r * (r - 1) / 2 * 81,
        3 => r * (r - 1) * (r - 2) / 6 * 729,
        _ => panic!(),
    };
    if k == 1 {
        tot += n[0];
    }
    if k == 2 {
        for i in 1..10 {
            for j in 1..m {
                for k in 1..10 {
                    let mut res = vec![0; m];
                    res[0] = i;
                    res[j] = k;
                    if res <= n {
                        tot += 1;
                    }
                }
            }
        }
    }
    if k == 3 {
        for i in 1..10 {
            for j in 1..m {
                for k in 1..10 {
                    for l in j + 1..m {
                        for o in 1..10 {
                            let mut res = vec![0; m];
                            res[0] = i;
                            res[j] = k;
                            res[l] = o;
                            if res <= n {
                                tot += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    puts!("{}\n", tot);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
