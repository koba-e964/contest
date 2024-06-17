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
    let m: i32 = get();
    let mut s = vec![0; n];
    for i in 0..n {
        let c = get_word();
        for (j, cc) in c.chars().enumerate() {
            if cc == 'o' {
                s[i] |= 1 << j;
            }
        }
    }
    let mut ans = n as u32;
    for bits in 1usize..1 << n {
        let mut x = 0;
        for i in 0..n {
            if bits >> i & 1 == 1 {
                x |= s[i];
            }
        }
        if x == (1 << m) - 1 {
            ans = ans.min(bits.count_ones());
        }
    }
    println!("{}", ans);
}
