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

fn pow(a: u32, mut e: usize) -> u32 {
    let mut prod = 1u32;
    let mut cur = a;
    while e > 0 {
        if e % 2 == 1 {
            prod = prod.wrapping_mul(cur);
        }
        cur = cur.wrapping_mul(cur);
        e /= 2;
    }
    prod
}

// https://yukicoder.me/problems/no/2344 (3.5)
// 30 段階減らす時、減らす前の 2 の倍数の差分が意味がなくなることを考えると、
// 長さ 31 になるまで mod 2 で簡約すれば良い。
fn main() {
    let t: usize = get();
    for _ in 0..t {
        let mut n: usize = get();
        let m: usize = get();
        let mut a: Vec<u32> = (0..n).map(|_| get()).collect();
        if n >= 31 {
            let rem = n - 31;
            for d in 0..18 {
                if (rem & 1 << d) == 0 {
                    continue;
                }
                let mut b = vec![0u32; n - (1 << d)];
                for i in 0..n - (1 << d) {
                    b[i] = pow(a[i], 1 << d).wrapping_add(pow(a[i + (1 << d)], 1 << d));
                }
                a = b;
                n -= 1 << d;
            }
        }
        assert!(n <= 31);
        while a.len() > 1 {
            let mut b = vec![0; a.len() - 1];
            for i in 0..a.len() - 1 {
                let tmp = a[i].wrapping_add(a[i + 1]);
                b[i] = tmp.wrapping_mul(tmp);
            }
            a = b;
        }
        println!("{}", a[0] & ((1 << m) - 1));
    }
}
