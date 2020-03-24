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

// Editorial
fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($($format:tt)*) => (let _ = write!(out,$($format)*););
    }
    input! {
        a: [usize],
    }
    const W: usize = 1_000_010;
    let mut fac = vec![0; W];
    for i in 0..W {
        fac[i] = i;
    }
    for i in 2..W {
        if fac[i] != i { continue; }
        for j in 2..(W - 1) / i + 1 {
            fac[i * j] = i;
        }
    }
    let mut a = a;
    a.sort();
    let mut g = vec![vec![]; W];
    let mut seen = HashSet::new();
    let mut is_two = false;
    for a in a {
        let mut ps = vec![];
        let mut v = a;
        while v > 1 {
            let mut e = 0;
            let p = fac[v];
            while v % p == 0 {
                v /= p;
                e += 1;
            }
            if e % 2 == 1 {
                ps.push(p);
            }
        }
        if ps.is_empty() {
            puts!("1\n");
            return;
        } else if ps.len() == 1 {
            g[1].push(ps[0]);
            g[ps[0]].push(1);
        } else {
            assert_eq!(ps.len(), 2);
            for i in 0..2 {
                g[ps[i]].push(ps[1 - i]);
            }
        }
        if seen.contains(&ps) {
            is_two = true;
        }
        seen.insert(ps);
    }
    if is_two {
        puts!("2\n");
        return;
    }
    // Shortest cycle
    const INF: i32 = 1 << 29;
    let mut mi = INF;
    for i in 2..1000 {
        if fac[i] != i { continue; }
        let mut que = VecDeque::new();
        que.push_back((i, 0, W));
        let mut dist = vec![INF; W];
        while let Some((v, d, pre)) = que.pop_front() {
            if dist[v] <= d {
                mi = min(mi, dist[v] + d);
                continue;
            }
            dist[v] = d;
            for &w in &g[v] {
                if pre == w { continue; }
                que.push_back((w, d + 1, v));
            }
        }
    }
    puts!("{}\n", if mi >= INF { -1 } else { mi });
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
