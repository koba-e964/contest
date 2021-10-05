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
    let x: usize = get();
    let mut ans = vec![];
    if x == 1 {
        if n % 4 == 1 {
            ans.extend(1..n + 1);
        } else if n % 4 == 2 {
            ans.extend(2..n + 2);
        } else if n % 4 == 3 {
            ans.extend(&[3, 5, 7]);
            ans.extend(8..n + 5);
        } else {
            ans.extend(&[1, 3, 4, 7]);
            ans.extend(8..n + 4);
        }
    } else {
        if n % 2 == 1 {
            let y = x ^ ((n / 2) & 1);
            ans.push(y);
            for i in 1..n / 2 + 1 {
                let tmp = (y / 2 - 1 + i) % 50002;
                ans.push(2 * tmp + 2);
                ans.push(2 * tmp + 3);
            }
        } else {
            let y = x ^ ((n / 2) & 1);
            ans.push(1);
            ans.push(y);
            for i in 1..n / 2 {
                let tmp = (y / 2 - 1 + i) % 50002;
                ans.push(2 * tmp + 2);
                ans.push(2 * tmp + 3);
            }
        }
    }
    for i in 0..n {
        println!("{}", ans[i]);
    }
}
