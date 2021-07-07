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
    let mut dat = vec![0; 2 * n];
    for i in 0..2 * n {
        dat[i] = i + 1;
    }
    for _ in 0..m {
        let k: usize = get();
        let mut ep = vec![0; 2 * n];
        if k == 0 {
            for i in 0..n {
                ep[2 * i] = dat[i];
                ep[2 * i + 1] = dat[i + n];
            }
        } else {
            for j in 0..2 * n {
                ep[j] = dat[(j + k) % (2 * n)];
            }
        }
        dat = ep;
    }
    for v in dat {
        println!("{}", v);
    }
}
