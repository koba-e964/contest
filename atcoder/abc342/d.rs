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

// Tags: square-free-part
fn main() {
    let n: usize = get();
    const W: usize = 200_100;
    let mut sqf = vec![0; W];
    for i in 1..W {
        sqf[i] = i;
    }
    for i in 2..W {
        if i * i >= W {
            break;
        }
        for j in 1..(W - 1) / (i * i) + 1 {
            while sqf[j * i * i] % (i * i) == 0 {
                sqf[j * i * i] /= i * i;
            }
        }
    }
    let mut zero = 0i64;
    let mut hm = std::collections::HashMap::new();
    let mut tot = 0i64;
    for _ in 0..n {
        let a: usize = get();
        if a == 0 {
            zero += 1;
            continue;
        }
        *hm.entry(sqf[a]).or_insert(0) += 1;
        tot += hm[&sqf[a]] - 1;
    }
    let nn = n as i64;
    println!("{}", tot + zero * (nn - zero) + zero * (zero - 1) / 2);
}
