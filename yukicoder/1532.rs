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

fn dfs(n: i64, k: i64, memo: &mut HashMap<(i64, i64), u64>) -> u64 {
    if let Some(&val) = memo.get(&(n, k)) {
        return val;
    }
    if k <= 0 {
        return 0;
    }
    if n <= 0 {
        return 0;
    }
    let mut sum = if k >= n {
        1
    } else {
        0
    };
    if k >= n {
        sum += dfs(n - 1, k / n, memo);
    }
    sum += dfs(n - 1, k, memo);
    memo.insert((n, k), sum);
    sum
}

fn main() {
    let n: i64 = get();
    let k: i64 = get();
    let mut memo = HashMap::new();
    println!("{}", dfs(n, k, &mut memo));
}
