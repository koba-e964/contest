use std::cmp::*;
use std::collections::*;
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
    ($next:expr,) => {};
    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => { ($(read_value!($next, $t)),*) };
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        h: usize, w: usize,
        a: [chars; h],
        n: usize,
        rce: [(usize1, usize1, i32); n],
    }
    let mut s = (0, 0);
    let mut t = (0, 0);
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 'S' {
                s = (i, j);
            }
            if a[i][j] == 'T' {
                t = (i, j);
            }
        }
    }
    let mut st = n;
    for i in 0..n {
        if (rce[i].0, rce[i].1) == s {
            st = i;
        }
    }
    if st == n {
        println!("No");
        return;
    }
    let mut rce = rce;
    rce.push((t.0, t.1, 0));
    const INF: i32 = 1 << 28;
    let mut dist = vec![vec![INF; n + 1]; n + 1];
    for i in 0..n {
        let mut dp = vec![vec![INF; w]; h];
        let mut que = VecDeque::new();
        que.push_back((rce[i].0, rce[i].1, 0));
        while let Some((x, y, d)) = que.pop_front() {
            if dp[x][y] <= d {
                continue;
            }
            dp[x][y] = d;
            for &(dx, dy) in &[(1i32, 0i32), (0, 1), (-1, 0), (0, -1)] {
                let nx = x.wrapping_add(dx as usize);
                let ny = y.wrapping_add(dy as usize);
                if nx < h && ny < w && a[nx][ny] != '#' {
                    que.push_back((nx, ny, d + 1));
                }
            }
        }
        for j in 0..n + 1 {
            dist[i][j] = if dp[rce[j].0][rce[j].1] <= rce[i].2 {
                0
            } else {
                INF
            };
        }
    }
    for k in 0..n + 1 {
        for i in 0..n + 1 {
            for j in 0..n + 1 {
                dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j]);
            }
        }
    }
    println!("{}", if dist[st][n] < INF { "Yes" } else { "No" });
}
