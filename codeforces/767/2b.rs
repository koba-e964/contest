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
    let t: usize = get();
    for _ in 0..t {
        let l: i64 = get();
        let r: i64 = get();
        let k: i64 = get();
        let to = r - l - k + 1;
        println!("{}", if (to == 1 && l >= 2) || r / 2 - (l - 1) / 2 >= to {
            "YES"
        } else {
            "NO"
        });
    }
}
