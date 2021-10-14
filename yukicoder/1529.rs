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

const MOD: i64 = 998_244_353;

fn main() {
    let n: usize = get();
    let mut pr = vec![true; n + 1];
    pr[0] = false;
    pr[1] = false;
    let mut ans = 1;
    for i in 2..n + 1 {
        if !pr[i] { continue; }
        for j in 2..n / i + 1 {
            pr[i * j] = false;
        }
        let mut e = 0;
        let mut v = n;
        while v % i == 0 {
            v /= i;
            e += 1;
        }
        if v == 1 {
            let tmp = i as i64 * i as i64 % MOD;
            for _ in 0..e - 1 {
                ans = ans * tmp % MOD;
            }
            continue;
        }
        while v >= i {
            v /= i;
            ans = ans * i as i64 % MOD;
        }
        let tmp = i as i64 * i as i64 % MOD;
        for _ in 0..e {
            ans = ans * tmp % MOD;
        }
    }
    println!("{}", ans);
}
