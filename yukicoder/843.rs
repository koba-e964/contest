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

fn main() {
    let n: usize = get();
    let mut pr = vec![true; n + 1];
    pr[0] = false;
    pr[1] = false;
    for i in 2..n + 1 {
        if !pr[i] { continue; }
        for j in 2..n / i + 1 {
            pr[i * j] = false;
        }
    }
    let mut tot = 0;
    for r in 2..n + 1 {
        if !pr[r] { continue; }
        if r * r - 2 <= n && pr[r * r - 2] {
            tot += if r == 2 { 1 } else { 2 };
        }
    }
    println!("{}", tot);
}
