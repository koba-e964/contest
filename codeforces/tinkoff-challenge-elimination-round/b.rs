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
        n: usize, m: usize,
        s: [chars; n],
    }
    let mut st = 0;
    let mut en = 0;
    let dxy = [(1i8, 0i8), (0, 1), (-1, 0), (0, -1)];
    const INF: i8 = 3;
    let mut dist = vec![INF; 4 * n * m];
    let mut que = std::collections::VecDeque::new();
    for i in 0..n {
        for j in 0..m {
            let v = i * m + j;
            if s[i][j] == 'S' {
                st = v;
            }
            if s[i][j] == 'T' {
                en = v;
            }
        }
    }
    for d in 0..4 {
        que.push_back((0, st + d * n * m));
    }
    while let Some((d, v)) = que.pop_front() {
        let x = (v / m) % n;
        let y = v % m;
        let dir = v / (m * n);
        if s[x][y] == '*' {
            continue;
        }
        if dist[v] <= d {
            continue;
        }
        dist[v] = d;
        let vred = v % (m * n);
        for d2 in 0..4 {
            if dir != d2 {
                let to = vred + d2 * n * m;
                if dist[to] > d + 1 {
                    dist[to] = d + 2;
                    que.push_back((d + 1, to));
                }
            }
        }
        let (dx, dy) = dxy[dir];
        let nx = x.wrapping_add(dx as usize);
        let ny = y.wrapping_add(dy as usize);
        if nx >= n || ny >= m { continue; }
        let to = nx * m + ny + dir * n * m;
        if dist[to] > d {
            dist[to] = d + 1;
            que.push_front((d, to));
        }
    }
    let mut mi = INF;
    for d in 0..4 {
        mi = min(mi, dist[en + d * m * n]);
    }
    puts!("{}\n", if mi <= 2 { "YES" } else { "NO" });
}

fn main() {
    solve();
}
