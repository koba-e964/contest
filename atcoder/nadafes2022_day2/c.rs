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

trait Change { fn chmax(&mut self, x: Self); fn chmin(&mut self, x: Self); }
impl<T: PartialOrd> Change for T {
    fn chmax(&mut self, x: T) { if *self < x { *self = x; } }
    fn chmin(&mut self, x: T) { if *self > x { *self = x; } }
}

fn main() {
    input! {
        h: usize, w: usize, n: usize,
        cdst: [(usize1, usize1, i64, i64); n],
        g: [chars; h],
    }
    let mut cdst = cdst;
    cdst.push((0, 0, 0, 0));
    const INF: i64 = 1 << 50;
    let mut dist = vec![vec![INF; n + 1]; n];
    for i in 0..n {
        let (c, d, _, _) = cdst[i];
        let mut tbl = vec![vec![INF; w]; h];
        let mut que = VecDeque::new();
        que.push_back((0, c, d));
        let dxy = [(0i32, 1i32), (0, -1), (-1, 0), (1, 0)];
        while let Some((d, x, y)) = que.pop_front() {
            if tbl[x][y] <= d { continue; }
            tbl[x][y] = d;
            for &(dx, dy) in &dxy {
                let nx = (x as i32 + dx) as usize;
                let ny = (y as i32 + dy) as usize;
                if nx >= h || ny >= w { continue; }
                if g[nx][ny] == '#' { continue; }
                que.push_back((d + 1, nx, ny));
            }
        }
        for j in 0..n + 1 {
            let (c, d, _, _) = cdst[j];
            dist[i][j] = tbl[c][d];
        }
    }
    let mut dp = vec![vec![INF; 1 << n]; n + 1];
    dp[n][0] = 0;
    for bits in 0..1 << n {
        for i in 0..n + 1 {
            let me = dp[i][bits];
            for j in 0..n {
                let (_, _, s, t) = cdst[j];
                if (bits & 1 << j) != 0 { continue; }
                let time = me + dist[j][i];
                let time = if s <= time {
                    time
                } else {
                    time + (s - time + 1) / 2 * 2
                };
                if time > t { continue; }
                dp[j][bits | 1 << j].chmin(time);
            }
        }
    }
    let mut ans = 0;
    for bits in 0..1 << n {
        for i in 0..n {
            if dp[i][bits] < INF {
                ans.chmax(bits.count_ones());
            }
        }
    }
    println!("{}", ans);
}
