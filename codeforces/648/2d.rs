#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Read;

#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

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

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

fn calc(n: usize, m: usize, mut a: Vec<Vec<char>>) -> bool {
    let dxy = [(0i32, 1i32), (1, 0), (0, -1), (-1, 0)];
    for i in 0..n {
        for j in 0..m {
            if a[i][j] != 'B' { continue; }
            for &(dx, dy) in &dxy {
                let nx = i.wrapping_add(dx as usize);
                let ny = j.wrapping_add(dy as usize);
                if nx >= n || ny >= m { continue; }
                if a[nx][ny] == 'G' { return false; }
                if a[nx][ny] == '.' {
                    a[nx][ny] = '#';
                }
            }
        }
    }
    let mut que = VecDeque::new();
    const INF: i32 = 1 << 28;
    let mut dist = vec![vec![INF; m]; n];
    if a[n - 1][m - 1] != '#' {
        que.push_back((0, n - 1, m - 1));
    }
    while let Some((d, x, y)) = que.pop_front() {
        if dist[x][y] <= d { continue; }
        dist[x][y] = d;
        for &(dx, dy) in &dxy {
            let nx = x.wrapping_add(dx as usize);
            let ny = y.wrapping_add(dy as usize);
            if nx >= n || ny >= m || a[nx][ny] == '#' { continue; }
            que.push_back((d + 1, nx, ny));
        }
    }
    for i in 0..n {
        for j in 0..m {
            if a[i][j] == 'G' && dist[i][j] >= INF {
                return false;
            }
            if a[i][j] == 'B' && dist[i][j] < INF {
                return false;
            }
        }
    }
    true
}

fn solve() {
    let t: usize = get();
    for _ in 0..t {
        let n: usize = get();
        let m: usize = get();
        let mut a = vec![vec![]; n];
        for i in 0..n {
            let s = get_word();
            a[i] = s.chars().collect();
        }
        println!("{}", if calc(n, m, a) { "Yes" } else { "No" });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
