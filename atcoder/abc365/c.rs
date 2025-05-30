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
    let n: usize = get();
    let m: i64 = get();
    let mut a: Vec<i64> = vec![0; n];
    for i in 0..n {
        a[i] = get();
    }
    let asum = a.iter().sum::<i64>();
    if asum <= m {
        println!("infinite");
        return;
    }
    let mut pass = 0;
    let mut fail = 1 << 30;
    while fail - pass > 1 {
        let mid = (pass + fail) / 2;
        let mut sum = 0;
        for i in 0..n {
            sum += a[i].min(mid);
        }
        if sum <= m {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    println!("{pass}");
}
