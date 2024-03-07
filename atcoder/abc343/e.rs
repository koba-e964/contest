use std::cmp::*;
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

fn vol((a1, a2): (i32, i32), (b1, b2): (i32, i32), (c1, c2): (i32, i32)) -> i32 {
    if a1 >= a2 || b1 >= b2 || c1 >= c2 {
        return 0;
    }
    (a2 - a1) * (b2 - b1) * (c2 - c1)
}

fn main() {
    let v1: i32 = get();
    let v2: i32 = get();
    let v3: i32 = get();
    if v1 + 2 * v2 + 3 * v3 != 1029 {
        println!("No");
        return;
    }
    for p1 in 0..3375 {
        let a1 = p1 / 225 - 7;
        let b1 = p1 / 15 % 15 - 7;
        let c1 = p1 % 15 - 7;
        for p2 in 0..3375 {
            let a2 = p2 / 225 - 7;
            let b2 = p2 / 15 % 15 - 7;
            let c2 = p2 % 15 - 7;
            let ami = max(0, max(a2, a1));
            let ama = 7 + min(min(a2, a1), 0);
            let bmi = max(0, max(b2, b1));
            let bma = 7 + min(min(b2, b1), 0);
            let cmi = max(0, max(c2, c1));
            let cma = 7 + min(min(c2, c1), 0);
            if vol((ami, ama), (bmi, bma), (cmi, cma)) != v3 {
                continue;
            }
            let mut common2 = -3 * v3;
            common2 += vol((max(a1, a2), 7 + min(a1, a2)), (max(b1, b2), 7 + min(b1, b2)),
                (max(c1, c2), 7 + min(c1, c2)));
            common2 += vol((max(a1, 0), 7 + min(a1, 0)), (max(b1, 0), 7 + min(b1, 0)),
                (max(c1, 0), 7 + min(c1, 0)));
            common2 += vol((max(a2, 0), 7 + min(a2, 0)), (max(b2, 0), 7 + min(b2, 0)),
                (max(c2, 0), 7 + min(c2, 0)));
            if common2 != v2 {
                continue;
            }
            println!("Yes\n0 0 0 {} {} {} {} {} {}", a2, b2, c2, a1, b1, c1);
            return;
        }
    }
    println!("No");
}
