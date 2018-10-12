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

const LINF: i64 = 1 << 58;

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    input!{
        n: usize,
        a: [[i32; n]; n],
    }
    let ni = n as i32;
    let mut pos = vec![0; n * n];
    for i in 0 .. n {
        for j in 0 .. n {
            pos[a[i][j] as usize - 1] = i * n + j;
        }
    }
    let mut kn = vec![vec![LINF >> 32; n * n]; n * n];
    let mut rk = vec![vec![LINF >> 32; n * n]; n * n];
    let mut bs = vec![vec![LINF >> 32; n * n]; n * n];
    for i in 0 .. n {
        for j in 0 .. n {
            for &(dx, dy)
                in &[(-2, -1), (-2, 1), (2, -1), (2, 1),
                     (1, 2), (-1, 2), (1, -2), (-1, -2)] {
                    let nx = i as i32 + dx;
                    let ny = j as i32 + dy;
                    if nx < 0 || nx >= ni || ny < 0 || ny >= ni { continue; }
                    let v = (nx * ni + ny) as usize;
                    kn[i * n + j][v] = 1;
                }
            for dx in -ni .. ni {
                let dy = dx;
                let nx = i as i32 + dx;
                let ny = j as i32 + dy;
                if nx < 0 || nx >= ni || ny < 0 || ny >= ni { continue; }
                let v = (nx * ni + ny) as usize;
                bs[i * n + j][v] = 1;
            }
            for dx in -ni .. ni {
                let dy = -dx;
                let nx = i as i32 + dx;
                let ny = j as i32 + dy;
                if nx < 0 || nx >= ni || ny < 0 || ny >= ni { continue; }
                let v = (nx * ni + ny) as usize;
                bs[i * n + j][v] = 1;
            }
            for u in 0 .. n {
                rk[i * n + j][u * n + j] = 1;
                rk[i * n + j][i * n + u] = 1;
            }
            kn[i * n + j][i * n + j] = 0;
            rk[i * n + j][i * n + j] = 0;
            bs[i * n + j][i * n + j] = 0;
        }
    }
    let mut dist = vec![vec![LINF; 3 * n * n]; 3 * n * n];
    for i in 0 .. n * n {
        for j in 0 .. n * n {
            dist[i][j] = kn[i][j] << 32;
            dist[n * n + i][n * n + j] = rk[i][j] << 32;
            dist[2 * n * n + i][2 * n * n + j] = bs[i][j] << 32;
        }
        for j in 0 .. 3 {
            for k in 0 .. 3 {
                if j != k {
                    dist[j * n * n + i][k * n * n + i] = 1 << 32 | 1;
                }
            }
        }
    }
    for k in 0 .. 3 * n * n {
        for i in 0 .. 3 * n * n {
            for j in 0 .. 3 * n * n {
                dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j]);
            }
        }
    }
    // eprintln!("dist = {:?}", dist);
    let mut dp = vec![[LINF; 3]; n * n];
    dp[0] = [0; 3];
    for i in 0 .. n * n - 1 {
        let src = pos[i];
        let dst = pos[i + 1];
        for j in 0 .. 3 {
            for k in 0 .. 3 {
                let cost: i64 = dist[j * n * n + src][k * n * n + dst];
                //eprintln!("{} -> {}", j * n * n + src, k * n * n + dst);
                //eprintln!("src = {}, dst = {}, cost = {}", src, dst, cost);
                dp[i + 1][k] = min(dp[i + 1][k], cost + dp[i][j]);
            }
        }
        //eprintln!("dp[{}] = {:?}", i + 1, dp[i + 1]);
    }
    let mut mi = LINF;
    for i in 0 .. 3 { mi = min(mi, dp[n * n - 1][i]); }
    puts!("{} {}\n", mi >> 32, mi as i32);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
