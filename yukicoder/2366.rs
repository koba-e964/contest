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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

// https://yukicoder.me/problems/no/2366 (3)
// T >= N+M-2 であれば単に移動したときの疲労度 0 が最適。T < N+M-2 としてよい。
// (地点, 時刻) を頂点としたグラフの、疲労度をコストとした時の最短路。
// 時刻としては T-(N+M-2) から N+M-2 まで考えれば良いはず。
// (i) T-(N+M-2) 以上: T-(N+M-2) 以下だとその後すぐゴールを目指すのが最適なので。
// (ii) N+M-2 以下: N+M-2 を越えると、その地点には直接行けばそれよりも短い時間で行けて、
// その時の疲労度は 0 であるため。
// k = N+M-2 とすると、頂点数は O(k^3) 程度で計算量は O(k^3 log k) 程度。
fn main() {
    input! {
        n: usize, m: usize, k: usize, t: usize,
        abcd: [(usize1, usize1, usize, i64); k],
    }
    if t >= n + m - 2 {
        println!("0");
        return;
    }
    let mut wonder = vec![vec![None; m]; n];
    for &(a, b, c, d) in &abcd {
        if c >= 2 {
            wonder[a][b] = Some((c - 1, d));
        }
    }
    const INF: i64 = 1 << 50;
    let tlim = 2 * (n + m - 2) - t + 1;
    let mut dist = vec![vec![vec![INF; tlim]; m]; n];
    let dxy = [(0i32, 1i32), (1, 0), (-1, 0), (0, -1)];
    let mut que = BinaryHeap::new();
    que.push((Reverse(0), 0, 0, n + m - 2 - t));
    while let Some((Reverse(d), x, y, t)) = que.pop() {
        if dist[x][y][t] <= d { continue; }
        dist[x][y][t] = d;
        if t + 1 < tlim {
            for &(dx, dy) in &dxy {
                let nx = (x as i32 + dx) as usize;
                let ny = (y as i32 + dy) as usize;
                if nx >= n || ny >= m { continue; }
                que.push((Reverse(d), nx, ny, t + 1));
            }
        }
        if let Some((cc, dd)) = wonder[x][y] {
            que.push((Reverse(d + dd), x, y, max(t, cc) - cc));
        }
    }
    let mut ans = INF;
    for i in 0..n + m - 1 {
        ans = min(ans, dist[n - 1][m - 1][i]);
    }
    println!("{}", if ans >= INF { -1 } else { ans });
}
