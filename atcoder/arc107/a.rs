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

fn f(x: i64) -> i64 {
    let x = x % MOD;
    x * (x + 1) % MOD
        * ((MOD + 1) / 2) % MOD
}

fn main() {
    let mut prod = 1;
    for _ in 0..3 {
        let a: i64 = get();
        prod = prod * f(a) % MOD;
    }
    println!("{}", prod);
}
