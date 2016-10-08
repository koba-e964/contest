#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::*;
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
}
fn get_word() -> String {
    let mut stdin = std::io::stdin();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.is_err() || u8b[0] <= ' ' as u8 {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = std::string::String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}
fn parse<T: std::str::FromStr>(s: &str) -> T { s.parse::<T>().ok().unwrap() }

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { parse(&get_word()) }

fn main() {
    let r: usize = get();
    let c: usize = get();
    let sy: usize = get::<usize>() - 1;
    let sx: usize = get::<usize>() - 1;
    let gy: usize = get::<usize>() - 1;
    let gx: usize = get::<usize>() - 1;

    let mut deq: VecDeque<(usize, usize, i32)> = VecDeque::new();
    let mut board: Vec<Vec<bool>> = vec![Vec::new(); r];
    let mut dp: Vec<Vec<i32>> = vec![Vec::new(); r];

    for i in 0 .. r {
        let s = get_word();
        let mut v = vec![false; c];
        let vd = vec![123456789; c];
        for j in 0 .. c {
           v[j] = s.chars().nth(j).unwrap() == '#';
        }
        board[i] = v;
        dp[i] = vd;
    }

    deq.push_back((sy, sx, 0));
    while let Some((a, b, dist)) = deq.pop_front() {
        let dxy: [i32; 5] = [0, -1, 0, 1, 0];
        if board[a][b] {
            continue;
        }
        if dp[a][b] <= dist {
            continue;
        }
        dp[a][b] = dist;
        for d in 0 .. 4 {
            let ny = (a as i32 + dxy[d]) as usize;
            let nx = (b as i32 + dxy[d + 1]) as usize;
            deq.push_back((ny, nx, dist + 1));
        }
    }
    println!("{}", dp[gy][gx]);
}
