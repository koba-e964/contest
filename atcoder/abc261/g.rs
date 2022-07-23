use std::cmp::*;
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
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// Tags: dp, polynomial-time
fn main() {
    input! {
        s: chars,
        t: chars,
        k: usize,
        ca: [(char, chars); k],
    }
    let m = t.len();
    let conv = |c| (c as u8 - b'a') as usize;
    let s: Vec<usize> = s.into_iter().map(conv).collect();
    let t: Vec<usize> = t.into_iter().map(conv).collect();
    let mut ca: Vec<(usize, Vec<usize>)> = ca.into_iter().map(|(c, a)| (conv(c), a.into_iter().map(conv).collect())).collect();
    ca.push((26, s));
    let k = ca.len();
    const INF: i64 = 1 << 50;
    let mut dist = vec![vec![INF; 27]; 27];
    for i in 0..27 {
        dist[i][i] = 0;
    }
    for i in 0..k {
        let (c, ref a) = ca[i];
        if a.len() == 1 {
            dist[c][a[0]] = 1;
        }
    }
    // shortest paths between one-letter strings
    for a in 0..27 {
        for i in 0..27 {
            for j in 0..27 {
                dist[i][j] = min(dist[i][j], dist[i][a] + dist[a][j]);
            }
        }
    }
    let mut dp = vec![vec![vec![INF; 27]; m + 1]; m + 1];
    for i in 0..m {
        dp[i][i + 1][t[i]] = 0;
    }
    let mut ep = vec![vec![vec![vec![INF; 51]; k]; m + 1]; m + 1];
    for i in 0..m {
        for j in 0..k {
            ep[i][i][j][0] = 0;
        }
    }
    // dp
    for len in 1..m + 1 {
        for i in 0..m - len + 1 {
            let j = i + len;
            for x in 0..k {
                let (c, ref a) = ca[x];
                for y in 1..a.len() + 1 {
                    for z in i..j {
                        let tmp = ep[i][z][x][y - 1] + dp[z][j][a[y - 1]];
                        ep[i][j][x][y] = min(ep[i][j][x][y], tmp);
                    }
                }
                dp[i][j][c] = min(dp[i][j][c], ep[i][j][x][a.len()] + 1);
            }
            for c in 0..27 {
                for d in 0..27 {
                    dp[i][j][d] = min(dp[i][j][d], dp[i][j][c] + dist[d][c]);
                }
            }
            for x in 0..k {
                let (_c, ref a) = ca[x];
                ep[i][j][x][1] = min(ep[i][j][x][1], dp[i][j][a[0]]);
            }
        }
    }
    if false {
        for i in 0..m {
            for j in i + 1..m + 1 {
                eprintln!("dp[{}][{}] = {:?}", i, j, &dp[i][j][..2]);
            }
        }
    }
    println!("{}", if dp[0][m][26] >= INF { -1 } else { dp[0][m][26] - 1 });
}
