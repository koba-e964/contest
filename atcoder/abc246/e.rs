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
    ($next:expr, usize1) => (read_value!($next, usize) - 1);
    ($next:expr, $t:ty) => ($next().parse::<$t>().expect("Parse error"));
}

fn main() {
    input! {
        n: usize,
        ax: usize1, ay: usize1,
        bx: usize1, by: usize1,
        s: [chars; n],
    }
    const INF: i64 = 1 << 50;
    let mut dist = vec![INF; 5 * n * n];
    let mut que = VecDeque::new();
    let dxy = [(1i32, 1i32), (-1, 1), (-1, -1), (1, -1)];
    que.push_back((0, 0, ax, ay));
    while let Some((d, z, x, y)) = que.pop_front() {
        let v = z * n * n + x * n + y;
        if dist[v] <= d { continue; }
        dist[v] = d;
        if z == 0 {
            for i in 1..5 {
                que.push_back((d + 1, i, x, y));
            }
        } else {
            let (dx, dy) = dxy[z - 1];
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;
            if nx < n && ny < n && s[nx][ny] != '#' {
                que.push_front((d, z, nx, ny));
            }
            que.push_front((d, 0, x, y));
        }
    }
    let ans = dist[n * bx + by];
    println!("{}", if ans >= INF { -1 } else { ans });
}
