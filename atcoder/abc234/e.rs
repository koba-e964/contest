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
    let x: i64 = get();
    let mut ans = 1i64 << 60;
    for d in 0..18 {
        for i in 0..10 {
            for j in -9..10 {
                let to = i + j * d;
                if to < 0 || to >= 10 { continue; }
                let mut v = 0i64;
                let mut c = i;
                for _ in 0..d + 1 {
                    v = 10 * v + c;
                    c += j;
                }
                if v >= x && v < ans {
                    ans = v;
                }
            }
        }
    }
    println!("{}", ans);
}
