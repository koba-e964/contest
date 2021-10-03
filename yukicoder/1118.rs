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
    let x: f64 = get();
    let mut s = 2.07079632677096370230;
    let n = x.floor() as i64;
    for i in 0..n {
        let y = i as f64;
        let tmp = if y != 0.0 {
            y.sin() / y
        } else {
            1.0
        };
        s -= tmp * tmp;
    }
    for i in n..1000000 + n {
        let y = x + i as f64 - n as f64;
        let z = i as f64;
        let tmp1 = if y != 0.0 {
            y.sin() / y
        } else {
            1.0
        };
        let tmp2 = if z != 0.0 {
            z.sin() / z
        } else {
            1.0
        };
        s += tmp1 * tmp1 - tmp2 * tmp2;
    }
    println!("{}", s);
}
