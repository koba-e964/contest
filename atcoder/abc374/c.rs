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
    let mut a: Vec<i64> = vec![0; n];
    for i in 0..n {
        a[i] = get();
    }
    let asum: i64 = a.iter().sum();
    let mut ans: i64 = asum;
    for bits in 0..1 << n {
        let mut sum: i64 = 0;
        for i in 0..n {
            if bits & 1 << i != 0 {
                sum += a[i];
            }
        }
        ans = ans.min(sum.max(asum - sum));
    }
    println!("{ans}");
}
