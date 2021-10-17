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

fn ask(a: &[usize]) -> usize {
    print!("?");
    for &a in a {
        print!(" {}", a);
    }
    println!();
    get()
}

fn main() {
    let n: usize = get();
    let mut p = vec![0; n];
    let mut filled = n + 1;
    for i in (0..n).rev() {
        if p[i] != 0 { continue; }
        let mut y = 1;
        let mut cons = vec![];
        let ceil = loop {
            let mut v = vec![1; n];
            v[i] = y + 1;
            let res = if y >= n {
                0
            } else {
                ask(&v)
            };
            if res == 0 || p[res - 1] != 0 || res - 1 == i {
                break filled;
            }
            cons.push((res - 1, y));
            y += 1;
        };
        p[i] = ceil - y;
        for &(pos, delta) in &cons {
            p[pos] = p[i] + delta;
        }
        filled = p[i];
    }
    print!("!");
    for i in 0..n {
        print!(" {}", p[i]);
    }
    println!();
}
