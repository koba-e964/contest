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
    let mut a = get_word().chars().collect::<Vec<_>>();
    let mut s = a.clone();
    for i in 0..n {
        let mut nxt = vec![];
        for j in 0..s.len() / 3 {
            let cnt = s[3 * j..3 * j + 3].iter().filter(|&&c| c != '0').count();
            nxt.push(if cnt <= 1 { '0' } else { '1'});
        }
        s = nxt;
    }
    if s[0] == '1' {
        for v in a.iter_mut() {
            *v = if *v == '1' { '0' } else { '1' };
        }
    }
    let mut s = a.iter().map(|&x| if x == '0' { 1 } else { 0 }).collect::<Vec<_>>();
    for i in 0..n {
        let mut nxt = vec![];
        for j in 0..s.len() / 3 {
            let mut tmp = s[3 * j..3 * j + 3].to_vec();
            tmp.sort_unstable();
            nxt.push(tmp[0] + tmp[1]);
        }
        s = nxt;
    }
    println!("{}", s[0]);
}
