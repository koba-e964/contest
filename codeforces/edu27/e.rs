#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::{Read, Write, BufWriter};
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

const INF: i64 = 1 << 50;

fn ok(xy: &[(i64, i64)], time: i64, n: i64, m: i64) -> bool {
    let mut xcoord = Vec::new();
    let mut ycoord = Vec::new();
    for &(x, y) in xy.iter() {
        let (xlo, xhi) = (max(1, x - time), min(n, x + time) + 1);
        let (ylo, yhi) = (max(1, y - time), min(m, y + time) + 1);
        xcoord.push(xlo);
        xcoord.push(xhi);
        ycoord.push(ylo);
        ycoord.push(yhi);
    }
    xcoord.push(1);
    xcoord.push(n + 1);
    ycoord.push(1);
    ycoord.push(m + 1);
    xcoord.sort_unstable();
    xcoord.dedup();
    ycoord.sort_unstable();
    ycoord.dedup();
    let h = xcoord.len();
    let w = ycoord.len();
    let mut tbl = vec![vec![0; w]; h];
    for &(x, y) in xy.iter() {
        let (xlo, xhi) = (max(1, x - time), min(n, x + time) + 1);
        let (ylo, yhi) = (max(1, y - time), min(m, y + time) + 1);
        let (xlo, xhi) = (xcoord.binary_search(&xlo).unwrap(), xcoord.binary_search(&xhi).unwrap());
        let (ylo, yhi) = (ycoord.binary_search(&ylo).unwrap(), ycoord.binary_search(&yhi).unwrap());
        tbl[xlo][ylo] += 1;
        tbl[xlo][yhi] -= 1;
        tbl[xhi][ylo] -= 1;
        tbl[xhi][yhi] += 1;
    }
    for i in 0 .. h {
        for j in 0 .. w {
            if i > 0 {
                tbl[i][j] += tbl[i - 1][j];
            }
            if j > 0 {
                tbl[i][j] += tbl[i][j - 1];
            }
            if i > 0 && j > 0 {
                tbl[i][j] -= tbl[i - 1][j - 1];
            }
        }
    }
    let mut xmi = INF;
    let mut xma = -INF;
    let mut ymi = INF;
    let mut yma = -INF;
    for i in 0 .. h - 1 {
        for j in 0 .. w - 1 {
            if tbl[i][j] == 0 {
                xmi = min(xmi, xcoord[i]);
                xma = max(xma, xcoord[i + 1]);
                ymi = min(ymi, ycoord[j]);
                yma = max(yma, ycoord[j + 1]);
            }
        }
    }
    if time <= 5 && false {
        eprintln!("time = {}", time);
        eprintln!("xcoord = {:?}", xcoord);
        eprintln!("ycoord = {:?}", ycoord);
        eprintln!("tbl = {:?}", tbl);
        eprintln!("x: [{}, {})", xmi, xma);
        eprintln!("y: [{}, {})", ymi, yma);
    }
    xma - xmi <= 2 * time + 1 && yma - ymi <= 2 * time + 1
}

fn solve() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {
        ($format:expr) => (write!(out,$format).unwrap());
        ($format:expr, $($args:expr),+) => (write!(out,$format,$($args),*).unwrap())
    }
    let n: i64 = get();
    let m: i64 = get();
    let k: usize = get();
    let mut xy = Vec::new();
    for _ in 0 .. k {
        let x: i64 = get();
        let y: i64 = get();
        xy.push((x, y));
    }
    let mut pass = max(n, m);
    let mut fail = -1;
    while pass - fail > 1 {
        let mid = (pass + fail) / 2;
        if ok(&xy, mid, n, m) {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    puts!("{}\n", pass);
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
