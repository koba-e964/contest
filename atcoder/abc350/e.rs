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

fn rec(n: i64, a: i64, x: f64, y: f64, memo: &mut HashMap<i64, f64>) -> f64 {
    if n == 0 {
        return 0.0;
    }
    if let Some(&v) = memo.get(&n) {
        return v;
    }
    let mut fst = 0.0;
    for i in 2..7 {
        fst += rec(n / i, a, x, y, memo);
    }
    fst /= 5.0;
    let mut mi = fst + y * 6.0 / 5.0;
    mi = mi.min(rec(n / a, a, x, y, memo) + x);
    memo.insert(n, mi);
    mi
}

fn main() {
    let n: i64 = get();
    let a: i64 = get();
    let x: f64 = get();
    let y: f64 = get();
    let mut memo = HashMap::new();
    println!("{}", rec(n, a, x, y, &mut memo));
}
