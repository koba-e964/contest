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

fn main() {
    let s = get_word().chars().collect::<Vec<_>>();
    let t = get_word().chars().collect::<Vec<_>>();
    for c in 1..s.len() {
        for w in c..s.len() {
            let mut v = vec![];
            for chk in s.chunks(w) {
                if chk.len() >= c {
                    v.push(chk[c - 1]);
                }
            }
            if t == v {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
