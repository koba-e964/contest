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

const W: i32 = 200;

fn ask(x: i32) -> i32 {
    let mut pts = vec![];
    for i in 1..W / x + 1 {
        for j in 1..W + 1 {
            pts.push(i * x);
            pts.push(j);
        }
    }
    println!("? {}", pts.len() / 2);
    for i in 0..pts.len() {
        print!("{}{}", pts[i], if i + 1 == pts.len() { "" } else { " " });
    }
    println!();
    get()
}

// The author solved this problem with hints.
fn main() {
    let area = ask(1);
    // Find optimal 2-adic valuation k along with the discrepancy at 2^{k+1}
    let mut pass = 0;
    let mut fail = 8;
    let mut pass_a = area;
    let mut fail_a = 0;
    while fail - pass > 1 {
        let mid = (fail + pass) / 2;
        let a = ask(1 << mid);
        if a << mid == area {
            pass = mid;
            pass_a = a;
        } else {
            fail = mid;
            fail_a = a;
        }
    }
    let w = (pass_a - fail_a * 2).abs();
    let h = area / w;
    eprintln!("h = {}, w = {}", h, w);
    println!("! {}", 2 * h + 2 * w - 4);
}
