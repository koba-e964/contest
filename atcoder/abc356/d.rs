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

fn main() {
    let mut n: i64 = get();
    let m: i64 = get();
    n += 1;
    const MOD: i64 = 998_244_353;
    let mut tot = 0;
    while n > 0 {
        let l = n & (n - 1);
        let height = (n - l - 1).count_ones() as i64;
        let x = (n - l) % MOD;
        tot += ((l & m) >> height).count_ones() as i64 * x % MOD;
        if height >= 1 {
            tot += (m & ((1 << height) - 1)).count_ones() as i64 * (((n - l) / 2) % MOD) % MOD;
        }
        tot %= MOD;
        n = l;
    }
    println!("{}", tot);
}
