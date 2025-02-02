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
    let q: usize = get();
    let mut pos = (0..n).collect::<Vec<_>>();
    let mut freq = vec![1; n];
    let mut mul = 0;
    for _ in 0..q {
        let ty: i32 = get();
        if ty == 1 {
            let p = get::<usize>() - 1;
            let h = get::<usize>() - 1;
            if freq[pos[p]] == 2 {
                mul -= 1;
            }
            freq[pos[p]] -= 1;
            freq[h] += 1;
            if freq[h] == 2 {
                mul += 1;
            }
            pos[p] = h;
        } else {
            println!("{mul}");
        }
    }
}
