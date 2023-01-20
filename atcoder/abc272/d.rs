use std::collections::*;
use std::io::Read;

fn get_word() -> String {
    let stdin = std::io::stdin();
    let mut stdin=stdin.lock();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.unwrap_or(0) == 0 || u8b[0] <= b' ' {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}

fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

fn nth(a: i64, n: i64) -> i64 {
    let mut pass = 0;
    let mut fail = std::cmp::min(a, 1 << ((60 + n - 1) / n)) + 1;
    while fail - pass > 1 {
        let mid = (fail + pass) / 2;
        let mut tmp = 1i64;
        for _ in 0..n {
            tmp = tmp.saturating_mul(mid);
        }
        if tmp <= a {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    pass
}

fn main() {
    let n: usize = get();
    let m: usize = get();
    let mut int = vec![];
    for i in -(n as i64 - 1)..n as i64 {
        if (m as i64) < i * i { continue; }
        let rem = m as i64 - i * i;
        let x = nth(rem, 2);
        if x * x == rem {
            int.push((i, x));
            if x > 0 {
                int.push((i, -x));
            }
        }
    }
    let mut que = VecDeque::new();
    que.push_back((0, 0, 0));
    const INF: i32 = 1 << 28;
    let mut dist = vec![vec![INF; n]; n];
    while let Some((d, x, y)) = que.pop_front() {
        if dist[x][y] <= d { continue; }
        dist[x][y] = d;
        for &(dx, dy) in &int {
            let nx = (x as i64 + dx) as usize;
            let ny = (y as i64 + dy) as usize;
            if nx < n && ny < n {
                que.push_back((d + 1, nx, ny));
            }
        }
    }
    for i in 0..n {
        for j in 0..n {
            print!("{}{}", if dist[i][j] >= INF { -1 } else { dist[i][j] }, if j + 1 == n { "\n" } else { " " });
        }
    }
}
