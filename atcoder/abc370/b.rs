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
    let mut a = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..i + 1 {
            a[i][j] = get::<usize>() - 1;
            a[j][i] = a[i][j];
        }
    }
    let mut x = 0;
    for i in 0..n {
        x = a[x][i];
    }
    println!("{}", x + 1);
}
