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
    let mut l: i64 = get();
    let r: i64 = get();
    let mut ans = vec![];
    while l < r {
        let mut i = 0;
        while l + (1 << i) <= r && l % (1 << i) == 0 {
            i += 1;
        }
        i -= 1;
        ans.push((l, l + (1 << i)));
        l += 1 << i;
    }
    println!("{}", ans.len());
    for (l, r) in ans {
        println!("{} {}", l, r);
    }
}
