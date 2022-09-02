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
    let k: usize = get();
    let mut v = vec![0i64; k];
    let mut r = vec![n; k];
    let ma = 100_000_000_000_000i64;
    v[0] = ma;
    r[0] = 1;
    for i in 0..n - 1 {
        v[2 * i + 1] = ma / 2 + i as i64 + 1;
        v[2 * i + 2] = ma / 2 - i as i64 - 1;
        r[2 * i + 1] = i + 2;
        r[2 * i + 2] = i + 2;
    }
    let mut sum = 0;
    for i in 2 * n - 1..k {
        v[i] = (i + 2 - 2 * n) as i64;
        sum += v[i];
    }
    v[2 * n - 2] -= sum;
    for i in 0..k {
        print!("{}{}", v[i], if i + 1 == k { "\n" } else { " " });
    }
    println!("YES");
    for i in 0..k {
        print!("{}{}", r[i], if i + 1 == k { "\n" } else { " " });
    }
}
