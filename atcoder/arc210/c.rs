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

fn calc(a: &[i64], m: i64) -> String {
    let n = a.len();
    let mut carry = vec![0; n];
    for i in 1..n {
        carry[i] = (carry[i - 1] + a[i - 1]) / 10;
    }
    let mut ans = vec![0; n];
    let mut borrow = 0;
    for i in (0..n).rev() {
        ans[i] = (a[i] + carry[i] - borrow) / m;
        let used = ans[i] * m;
        borrow = 0.max(used + borrow - a[i]) * 10;
    }
    if n <= 30 {
        eprintln!("ans = {ans:?}");
    }
    let mut pos = 0;
    loop {
        if pos >= ans.len() {
            break;
        }
        let q = ans[pos] / 10;
        ans[pos] -= q * 10;
        if q > 0 {
            if pos + 1 >= ans.len() {
                ans.push(0);
            }
            ans[pos + 1] += q;
        }
        pos += 1;
    }
    pos = ans.len();
    while pos > 0 && ans[pos - 1] == 0 {
        pos -= 1;
    }
    if pos == 0 {
        return "0".to_string();
    }
    let mut s = "".to_string();
    for i in (0..pos).rev() {
        s.push((b'0' + ans[i] as u8) as char);
    }
    s
}

fn main() {
    let t: i32 = get();
    for _ in 0..t {
        let n: usize = get();
        let m: i64 = get();
        let a = (0..n).map(|_| get::<i64>()).collect::<Vec<_>>();
        println!("{}", calc(&a, m));
    }
}
