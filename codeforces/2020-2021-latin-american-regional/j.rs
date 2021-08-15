use std::io::{BufWriter, Write, Read};

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

fn calc(v: &[usize]) -> Result<usize, usize> {
    let mut a = 0;
    for &v in v {
        a += 1 << (3 * v);
    }
    if a == 1 << (3 * v[0] + 3) && v.len() == 8 {
        return Err(v[0]);
    }
    Ok(a)
}

// Tags: constant-factor-optimization
fn main() {
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    let n: usize = get();
    let k: usize = get();
    let mut dp = vec![0; 1 << 24];
    let mut all = vec![0; k];
    let mut old = vec![];
    let mut set4 = vec![];
    for i in 0usize..1 << 12 {
        let mut u = 0;
        for j in 0..4 {
            u += (i >> (3 * j)) & 7;
        }
        if u <= 8 {
            set4.push(i);
        }
    }
    eprintln!("|set4| = {}", set4.len());
    for _ in 0..n {
        let ty: char = get();
        if ty == 'C' {
            let len: usize = get();
            let v: Vec<usize> = (0..len).map(|_| get::<usize>() - 1)
                .collect();
            let tmp = calc(&v);
            old.push(tmp);
            match tmp {
                Ok(v) => {
                    let bb = v & !4095;
                    for &i in &set4 {
                        let mut ok = true;
                        for j in 0..4 {
                            if ((v >> (3 * j)) & 7) < ((i >> (3 * j)) & 7) {
                                ok = false;
                            }
                        }
                        if ok { dp[bb + i] += 1; }
                    }
                }
                Err(idx) => all[idx] += 1,
            }
        } else if ty == 'J' {
            let len: usize = get();
            let v: Vec<usize> = (0..len).map(|_| get::<usize>() - 1)
                .collect();
            let mut ans = 0;
            match calc(&v) {
                Ok(v) => {
                    let bb = v & 4095;
                    for &i in &set4 {
                        let mut ok = true;
                        for j in 4..8 {
                            if ((v >> (3 * j)) & 7) > ((i >> (3 * j - 12)) & 7) {
                                ok = false;
                            }
                        }
                        if ok { ans += dp[bb | i << 12]; }
                    }
                    for i in 0..k {
                        let mask = 7 << (3 * i);
                        if (v & mask) == v {
                            ans += all[i];
                        }
                    }
                }
                Err(idx) => ans = all[idx],
            }
            puts!("{}\n", ans);
        } else {
            assert_eq!(ty, 'D');
            let idx = get::<usize>() - 1;
            let tmp = old[idx];
            match tmp {
                Ok(v) => {
                    let bb = v & !4095;
                    for &i in &set4 {
                        let mut ok = true;
                        for j in 0..4 {
                            if ((v >> (3 * j)) & 7) < ((i >> (3 * j)) & 7) {
                                ok = false;
                            }
                        }
                        if ok { dp[bb + i] -= 1; }
                    }
                }
                Err(idx) => all[idx] -= 1,
            }
        }
    }
}
