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

fn kado(d: [char; 3]) -> bool {
    (d[0] < d[1] && d[1] > d[2]) || (d[0] > d[1] && d[1] < d[2])
}

fn main() {
    let c1: char = get();
    let c2: char = get();
    let c3: char = get();
    let c = [c1, c2, c3];
    let mut ans = "".to_string();
    for i in 0..3 {
        if c[i] == '?' {
            let mut d = c;
            d[i] = '1';
            if kado(d) {
                ans.push('1');
            }
            d[i] = '4';
            if kado(d) {
                ans.push('4');
            }
        }
    }
    println!("{}", ans);
}
