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
    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };
    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        h: usize, w: usize, t: i32,
        a: [chars; h],
    }
    let mut cand = vec![];
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 'o' {
                cand.push((i, j));
            }
        }
    }
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 'S' {
                cand.push((i, j));
            }
        }
    }
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 'G' {
                cand.push((i, j));
            }
        }
    }
    let m = cand.len();
    let mut inter = vec![vec![t + 1; m]; m];
    let dxy = [(1i32, 0i32), (0, -1), (-1, 0), (0, 1)];
    for i in 0..m {
        let mut dist = vec![vec![t + 1; w]; h];
        let (x, y) = cand[i];
        let mut que = VecDeque::new();
        que.push_back((0, x, y));
        while let Some((d, x, y)) = que.pop_front() {
            if dist[x][y] <= d { continue; }
            dist[x][y] = d;
            for &(dx, dy) in &dxy {
                let nx = (x as i32 + dx) as usize;
                let ny = (y as i32 + dy) as usize;
                if nx >= h || ny >= w {
                    continue;
                }
                if a[nx][ny] == '#' { continue; }
                que.push_back((d + 1, nx, ny));
            }
        }
        for j in 0..m {
            let (z, w) = cand[j];
            inter[i][j] = dist[z][w];
        }
    }
    let mut dp = vec![vec![t + 1; 1 << (m - 2)]; m - 1];
    dp[m - 2][0] = 0;
    for bits in 1..1 << (m - 2) {
        for i in 0..m - 2 {
            if (bits & 1 << i) == 0 { continue; }
            let mut me = t + 1;
            for j in 0..m - 1 {
                me = min(me, dp[j][bits ^ 1 << i] + inter[i][j]);
            }
            dp[i][bits] = me;
        }
    }
    let mut ans = -1;
    for bits in 0usize..1 << (m - 2) {
        let bc = bits.count_ones() as i32;
        for i in 0..m - 1 {
            if dp[i][bits] + inter[i][m - 1] <= t {
                ans = max(ans, bc);
            }
        }
    }
    println!("{}", ans);
}
