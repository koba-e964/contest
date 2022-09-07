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
    let names: Vec<_> = (0..n).map(|_| get_word()).collect();
    let mut coo = names.clone();
    coo.sort();
    let mut good = vec![];
    let mut bad = vec![];
    for i in 0..n {
        let idx = coo.binary_search(&names[i]).unwrap();
        let u = get_word();
        let u = coo.binary_search(&u).unwrap();
        get_word(); get_word();
        let kind = get_word();
        get_word();
        let val = 1 << (n - 1 - idx) | 1 << (n - 1 - u);
        if kind == "good" {
            if idx != u {
                good.push(val);
            }
        } else {
            if idx == u {
                println!("No answers");
                return;
            }
            bad.push(val);
        }
    }
    let mut ma = (0, 0);
    for bits in 1u32..1 << n {
        let mut ok = true;
        for &g in &good {
            let tmp = g & bits;
            if tmp.count_ones() == 1 { ok = false; }
        }
        for &b in &bad {
            let tmp = b & bits;
            if tmp.count_ones() != 1 { ok = false; }
        }
        if ok {
            ma = std::cmp::max(ma, (bits.count_ones(), bits));
        }
    }
    if ma.0 == 0 {
        println!("No answers");
        return;
    }
    for i in 0..n {
        if (ma.1 & 1 << (n - 1 - i)) != 0 {
            println!("{}", coo[i]);
        }
    }
}
