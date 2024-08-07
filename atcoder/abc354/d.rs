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

fn calc(a: i64, b: i64) -> i64 {
    let ra = (a % 4 + 4) % 4;
    let rb = (b % 2 + 2) % 2;
    let qa = (a - ra) / 4;
    let qb = (b - rb) / 2;
    let mut ans = 8 * qa * qb;
    ans += qb * [0, 3, 6, 7][ra as usize];
    ans += 4 * qa * rb;
    ans += [[0; 4], [0, 2, 3, 3]][rb as usize][ra as usize];
    ans
}

fn main() {
    let a: i64 = get();
    let b: i64 = get();
    let c: i64 = get();
    let d: i64 = get();
    println!("{}", calc(c, d) - calc(a, d) - calc(c, b) + calc(a, b));
}
