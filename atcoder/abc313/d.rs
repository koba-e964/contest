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
    let k: usize = get();
    let mut whole = 0;
    let mut diff = vec![0; n - 1];
    let mut last = 0;
    for i in k - 1..n {
        print!("?");
        for j in 0..k - 1 {
            print!(" {}", j + 1);
        }
        println!(" {}", i + 1);
        let res: i32 = get();
        if i == k - 1 {
            whole = res;
        } else {
            diff[i - 1] = last ^ res;
        }
        last = res;
    }
    last = whole ^ diff[k - 1];
    for i in (0..k - 1).rev() {
        print!("?");
        for j in 0..i {
            print!(" {}", j + 1);
        }
        for j in i + 1..k + 1 {
            print!(" {}", j + 1);
        }
        println!();
        let res: i32 = get();
        diff[i] = res ^ last;
        last = res;
    }
    let mut d = 0;
    last = 0;
    for i in 0..k - 1 {
        last ^= diff[i];
        d ^= last;
    }
    let fst = whole ^ d;
    print!("! {}", fst);
    last = fst;
    for i in 1..n {
        last ^= diff[i - 1];
        print!(" {}", last);
    }
    println!();
}
