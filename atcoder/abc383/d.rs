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
    const W: usize = 2_000_001;
    let mut is_prime = vec![true; W];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..W {
        if is_prime[i] {
            let mut j = i * 2;
            while j < W {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    let n: i64 = get();
    let mut ans = 0;
    for p in 2..W {
        if !is_prime[p] {
            continue;
        }
        let p8 = p as i64 * p as i64;
        let p8 = p8.saturating_mul(p8);
        let p8 = p8.saturating_mul(p8);
        if p8 <= n {
            ans += 1;
        }
        for q in p + 1..(W - 1) / p + 1 {
            if !is_prime[q] {
                continue;
            }
            let pq = p as i64 * q as i64;
            if pq * pq <= n {
                ans += 1;
            }
        }
    }
    println!("{ans}");
}
