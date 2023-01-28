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

fn factorize(mut x: i64) -> Vec<(i64, usize)> {
    let mut p = 2;
    let mut ans = vec![];
    while p * p <= x {
        let mut e = 0;
        while x % p == 0 {
            x /= p;
            e += 1;
        }
        if e > 0 {
            ans.push((p, e));
        }
        p += 1;
    }
    if x > 1 {
        ans.push((x, 1));
    }
    ans
}

fn main() {
    let k: i64 = get();
    let pe = factorize(k);
    let mut pass = k;
    let mut fail = 1;
    while pass - fail > 1 {
        let mid = (pass + fail) / 2;
        let ok = pe.iter().all(|&(p, e)| {
            let mut x = mid;
            let mut y = 0;
            while x > 0 {
                x /= p;
                y += x;
            }
            y >= e as i64
        });
        if ok {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    println!("{}", pass);
}
