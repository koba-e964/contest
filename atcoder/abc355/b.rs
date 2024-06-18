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
    let m: usize = get();
    let a = (0..n).map(|_| get::<i32>()).collect::<Vec<_>>();
    let b = (0..m).map(|_| get::<i32>()).collect::<Vec<_>>();
    let mut c = vec![];
    for a in a {
        c.push((a, 0));
    }
    for b in b {
        c.push((b, 1));
    }
    c.sort();
    for i in 0..c.len() - 1 {
        if c[i].1 + c[i + 1].1 == 0 {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
