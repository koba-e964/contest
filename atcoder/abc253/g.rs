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

fn conv(n: usize, mut x: i64) -> (usize, usize) {
    for i in 0..n {
        if x > (n - i - 1) as i64 {
            x -= (n - i - 1) as i64;
            continue;
        }
        return (i, i + x as usize);
    }
    panic!();
}

fn f(n: usize, (lx, ly): (usize, usize), (rx, ry): (usize, usize)) -> Vec<usize> {
    let mut p: Vec<usize> = (0..n).collect();
    if lx == rx {
        for i in ly..ry + 1 {
            p.swap(lx, i);
        }
        return p;
    }
    for i in ly..n {
        p.swap(lx, i);
    }
    // rot_right(p[i..n]) for i in lx + 1..rx
    p[lx + 1..].rotate_right(rx - lx - 1);
    p[lx + 1..rx].reverse();
    for i in rx + 1..ry + 1 {
        p.swap(rx, i);
    }
    p
}

fn main() {
    let n: usize = get();
    let l: i64 = get();
    let r: i64 = get();
    let l = conv(n, l);
    let r = conv(n, r);
    let ans = f(n, l, r);
    for i in 0..n {
        print!("{}{}", ans[i] + 1, if i + 1 == n { "\n" } else { " " });
    }
}
