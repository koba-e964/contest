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
    let t: i32 = get();
    for _ in 0..t {
        let n: usize = get();
        let m: i64 = get();
        let mut a = vec![0i64; n];
        let mut b = vec![0i64; n];
        let mut ans = 0i64;
        for i in 0..n {
            a[i] = get();
            ans += a[i];
        }
        for i in 0..n {
            b[i] = get();
            ans += b[i];
        }
        a.sort_unstable(); a.reverse();
        b.sort_unstable();
        let mut pos = 0;
        for a in a {
            while pos < n && a + b[pos] < m {
                pos += 1;
            }
            if pos < n {
                ans -= m;
                pos += 1;
            }
        }
        println!("{ans}");
    }
}
