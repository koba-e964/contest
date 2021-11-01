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

fn main() {
    let n: usize = get();
    let k: usize = get();
    if k == 1 || k == n - 1 {
        println!("No");
        return;
    }
    if k >= 3 {
        println!("Yes");
        println!("1 2 -{}", 2 * k - 1);
        for i in 1..k - 1 {
            println!("{} {} 2", i + 1, i + 2);
        }
        for i in k + 1..n + 1 {
            println!("{} {} 2", k, i);
        }
        return;
    }
    if n % 2 != 0 {
        println!("No");
        return;
    }
    println!("Yes");
    for i in 0..n - 1 {
        println!("{} {} {}", i + 1, i + 2, if i % 2 == 0 {
            1_000_000
        } else {
            -1_000_001
        });
    }
}
