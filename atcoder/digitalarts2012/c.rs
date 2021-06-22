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

fn main() {
    let n: usize = get();
    let m: usize = get();
    let k: usize = get();
    let mut count = vec![0i64; n];
    let mut chro = vec![0i64; n];
    let mut since = vec![HashMap::new(); n];
    for _ in 0..m {
        let c: char = get();
        let j = get::<usize>() - 1;
        match c {
            't' => {
                chro[j] += 1;
            }
            'f' => {
                let k = get::<usize>() - 1;
                since[j].insert(k, chro[j]);
                since[k].insert(j, chro[k]);
            }
            'u' => {
                let k = get::<usize>() - 1;
                let s = since[j][&k];
                count[k] += chro[j] - s;
                since[j].remove(&k);
                let s = since[k][&j];
                count[j] += chro[k] - s;
                since[k].remove(&j);
            }
            _ => panic!(),
        }
    }
    for i in 0..n {
        for (&idx, &s) in &since[i] {
            count[idx] += chro[i] - s;
        }
        count[i] += chro[i];
    }
    count.sort();
    println!("{}", count[n - k]);
}
