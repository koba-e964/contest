fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

const MOD: i64 = 998_244_353;

fn powmod(x: i64, mut e: i64) -> i64 {
    let mut sum = 1;
    let mut cur = x % MOD;
    while e > 0 {
        if e % 2 != 0 {
            sum = sum * cur % MOD;
        }
        cur = cur * cur % MOD;
        e /= 2;
    }
    sum
}
fn main() {
    let s: Vec<_> = getline().trim().bytes().collect();
    let n = s.len();
    let mut tot = 0;
    for i in 0..n {
        let tmp = powmod(11, (n - i - 1) as i64) * powmod(2, i as i64) % MOD;
        let idx = (s[i] - b'0') as i64;
        tot = (tot + tmp * idx) % MOD;
    }
    println!("{}", tot);
}
