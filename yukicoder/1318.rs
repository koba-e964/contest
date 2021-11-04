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
    let m: usize = get();
    let mut dat = vec![vec![]; 2 * m + 1];
    for a in 0..m + 1 {
        for b in 0..a {
            dat[a + b].push((a * a + b * b, 2));
        }
        dat[2 * a].push((2 * a * a, 1));
    }
    for i in 0..2 * m + 1 {
        dat[i].sort();
    }
    let mut ans = vec![0i64; n + 1];
    for sum in 0..2 * m + 1 {
        for &(sqsum, v) in &dat[sum] {
            if sum * sum + sqsum > 2 * n {
                continue;
            }
            for cd in 0..2 * m + 1 {
                let tmp = sqsum + (sum + cd) * (sum + cd);
                if 2 * n < tmp {
                    break;
                }
                for &(c2d2, w) in &dat[cd] {
                    if c2d2 + tmp > 2 * n {
                        break;
                    }
                    ans[(c2d2 + tmp) / 2] += v * w;
                }
            }
        }
    }
    for i in 0..n + 1 {
        println!("{}", ans[i]);
    }
}
