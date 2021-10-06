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

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

fn ask(a: usize, b: usize) -> i32 {
    println!("{} {}", a + 1, b + 1);
    let s = get_word();
    if s == "Black" {
        1
    } else if s == "White" {
        0
    } else {
        panic!();
    }
}

fn main() {
    let n: usize = get();
    let _m: usize = get();
    let mut col = vec![vec![2; n]; n];
    col[0][0] = 1;
    col[n - 1][n - 1] = 1;
    let mut que = VecDeque::new();
    que.push_back((0, 0));
    let dxy = [(0i32, 1i32), (1, 0), (0, -1), (-1, 0)];
    let mut vis = vec![vec![false; n]; n];
    while let Some((x, y)) = que.pop_front() {
        if vis[x][y] {
            continue;
        }
        vis[x][y] = true;
        for &(dx, dy) in &dxy {
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;
            if nx >= n || ny >= n { continue; }
            if col[nx][ny] == 2 {
                col[nx][ny] = ask(nx, ny);
            }
            if col[nx][ny] == 1 {
                que.push_back((nx, ny));
            }
        }
    }
    println!("{}", if vis[n - 1][n - 1] { "Yes" } else { "No" });
}
