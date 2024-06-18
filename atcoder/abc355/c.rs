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
    let t: i32 = get();
    let mut ans = -1;
    let mut row = vec![0; n];
    let mut col = vec![0; n];
    let mut diag0 = 0;
    let mut diag1 = 0;
    for turn in 1..t + 1 {
        let x = get::<usize>() - 1;
        row[x / n] += 1;
        if row[x / n] == n {
            ans = turn;
        }
        col[x % n] += 1;
        if col[x % n] == n {
            ans = turn;
        }
        if x / n == x % n {
            diag0 += 1;
            if diag0 == n {
                ans = turn;
            }
        }
        if x / n + x % n == n - 1 {
            diag1 += 1;
            if diag1 == n {
                ans = turn;
            }
        }
        if ans != -1 {
            break;
        }
    }
    println!("{}", ans);
}
