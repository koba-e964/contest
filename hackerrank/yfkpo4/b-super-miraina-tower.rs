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

const LIM: i64 = 679891637638612258;

fn main() {
    let n: i64 = get();
    let mut fib = vec![1, 2];
    while *fib.last().unwrap() < LIM {
        let x = fib[fib.len() - 2] + fib[fib.len() - 1];
        fib.push(x);
    }
    let mut tmp = 0;
    for i in 0..fib.len() - 1 {
        let c = std::cmp::min(fib[i + 1], n) - fib[i];
        if c < 0 { break; }
        if i % 2 == 0 {
            tmp -= c;
        } else {
            tmp += c;
        }
    }
    println!("{}", tmp);
}
