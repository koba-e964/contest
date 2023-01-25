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
    let parts: [usize; 9] = [4, 9, 5, 7, 11, 13, 17, 19, 23];
    let mut acc = vec![0; 10];
    for i in 0..9 {
        acc[i + 1] = acc[i] + parts[i];
    }
    let m: usize = parts.iter().sum();
    println!("{}", m);
    let mut a = vec![0; m];
    for i in 0..9 {
        for j in 0..parts[i] {
            a[acc[i] + j] = acc[i] + (j + 1) % parts[i] + 1;
        }
    }
    for i in 0..m {
        print!("{}{}", a[i], if i + 1 == m { "\n" } else { " " });
    }
    let mut b = vec![0usize; m];
    for i in 0..m {
        b[i] = get();
    }
    let mut n = 0;
    let mut mo = 1;
    for i in 0..9 {
        let x = b[acc[i]] - acc[i] - 1;
        let y = parts[i];
        let mut newn = 0;
        for j in 0..y {
            if (n + mo * j) % y == x {
                newn = n + mo * j;
                break;
            }
        }
        n = newn;
        mo *= y;
    }
    println!("{}", n);
}
