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

fn dfs(n: usize, m: usize, pre: usize, p: &mut Vec<usize>) {
    if p.len() >= n {
        for i in 0..n {
            print!("{}{}", p[i], if i + 1 == n { "\n" } else { " " });
        }
        return;
    }
    for i in pre + 1..m + 1 {
        p.push(i);
        dfs(n, m, i, p);
        p.pop();
    }
}

fn main() {
    let n: usize = get();
    let m: usize = get();
    dfs(n, m, 0, &mut vec![]);
}
