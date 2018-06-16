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
    let mut stdin = std::io::stdin();
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

fn solve() {
    let k = get();
    let mut a: Vec<Vec<i64>> = vec![Vec::new(); k];
    let mut tapris: Vec<Vec<i64>> = vec![Vec::new(); k];
    for i in 0 .. k {
        let n = get();
        let v: Vec<i64> = (0 .. n).map(|_| get()).collect();
        let sum: i64 = v.iter().sum();
        a[i] = v;
        tapris[i] = vec![0; n];
        for j in 0 .. n { tapris[i][j] = sum - a[i][j]; }
    }
    let mut hm: HashMap<i64, _> = HashMap::new();
    for i in 0 .. k {
        for j in 0 .. tapris[i].len() {
            let tmp = hm.get(&tapris[i][j]).cloned();
            if let Some((x, y)) = tmp {
                if x != i {
                    println!("YES");
                    println!("{} {}", i + 1, j + 1);
                    println!("{} {}", x + 1, y + 1);
                    return;
                }
            } else {
                hm.insert(tapris[i][j], (i, j));
            }
        }
    }
    println!("NO");
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
