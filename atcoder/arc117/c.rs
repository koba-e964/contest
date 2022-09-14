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

fn comb_mod3(mut a: usize, mut b: usize) -> usize {
    let mut prod = 1;
    while a > 0 && b > 0 {
        let x = a % 3;
        let y = b % 3;
        if x < y { return 0; }
        if (x, y) == (2, 1) { prod = 3 - prod; }
        a /= 3;
        b /= 3;
    }
    if b == 0 { return prod; }
    0
}

fn main() {
    let n: usize = get();
    let c: Vec<_> = get_word().bytes().collect();
    let tbl = [b'B', b'W', b'R'];
    let mut ans = 0;
    for i in 0..n {
        let idx = tbl.iter().position(|&x| x == c[i]).unwrap();
        ans = (ans + idx * comb_mod3(n - 1, i)) % 3;
    }
    if n % 2 == 0 {
        ans = (3 - ans) % 3;
    }
    println!("{}", tbl[ans] as char);
}
