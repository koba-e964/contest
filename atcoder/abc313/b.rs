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
    let m: i32 = get();
    let mut indeg = vec![0; n];
    for _ in 0..m {
        let _a = get::<usize>() - 1;
        let b = get::<usize>() - 1;
        indeg[b] += 1;
    }
    let mut ans = -1;
    let c = indeg.iter().filter(|&&x| x == 0).count();
    for i in 0..n {
        if indeg[i] == 0 {
            ans = i as i32 + 1;
        }
    }
    println!("{}", if c == 1 { ans } else { -1 });
}
