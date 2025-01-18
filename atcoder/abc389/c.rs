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
    let q: i32 = get();
    let mut dat = vec![0];
    let mut offset = 1;
    for _ in 0..q {
        let ty: i32 = get();
        if ty == 1 {
            let l: i64 = get();
            let last = dat[dat.len() - 1];
            dat.push(last + l);
        } else if ty == 2 {
            offset += 1;
        } else {
            let k: usize = get();
            println!("{}", dat[offset + k - 2] - dat[offset - 1]);
        }
    }
}
