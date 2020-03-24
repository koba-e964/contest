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

// O(nd)
fn calc(n: usize, d: usize) -> Option<Vec<usize>> {
    let mut rem = n;
    let mut layers = vec![];
    let mut cur = 1;
    let mut sum = 0;
    while rem > 0 {
        let now = min(rem, cur);
        sum += layers.len() * now;
        layers.push(now);
        rem -= now;
        cur *= 2;
    }
    if sum > d {
        return None;
    }
    while sum < d {
        let mut nxt = None;
        for i in (0..layers.len()).rev() {
            if layers[i] >= 2 {
                nxt = Some(i);
                break;
            }
        }
        if let Some(nxt) = nxt {
            layers[nxt] -= 1;
            if layers.len() <= nxt + 1 {
                layers.push(0);
            }
            layers[nxt + 1] += 1;
        } else {
            return None;
        }
        sum += 1;
    }
    Some(layers)
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        nd: [(usize, usize)],
    }
    for (n, d) in nd {
        let t = if let Some(t) = calc(n, d) {
            t
        } else {
            puts!("NO\n");
            continue;
        };
        let mut ans = vec![0; n];
        let mut layer = vec![];
        let mut cur = 0;
        for i in 0..t.len() {
            let mut now = vec![];
            for j in 0..t[i] {
                now.push(cur + j);
            }
            layer.push(now);
            if i > 0 {
                for j in 0..t[i] {
                    ans[cur + j] = layer[i - 1][j / 2];
                }
            }
            cur += t[i];
        }
        puts!("YES\n");
        for i in 1..n {
            puts!("{}{}", ans[i] + 1, if i + 1 == n { "\n" } else { " " });
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
