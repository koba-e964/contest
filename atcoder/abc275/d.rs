use std::io::Read;
use std::collections::*;

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

fn rec(a: i64, memo: &mut HashMap<i64, i64>) -> i64 {
    if a == 0 { return 1; }
    if let Some(&v) = memo.get(&a) {
        return v;
    }
    let val = rec(a / 2, memo) + rec(a / 3, memo);
    memo.insert(a, val);
    val
}

fn main() {
    let n: i64 = get();
    let mut memo = HashMap::new();
    println!("{}", rec(n, &mut memo));
}
