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

// https://yukicoder.me/problems/no/2255 (3.5)
// p != 2 であれば 0。p == 2 のとき、すべての変数を含む項がなければ、項ごとの和が 0 なので 0。
// すべての変数を含む項があるとき、それらの和は変数を取り除いた部分行列の行列式で決まる。
// Tags: gaussian-elimination-mod-2, determinant
fn main() {
    let t: usize = get();
    for _ in 0..t {
        let n: usize = get();
        let p: i32 = get();
        let mut a = vec![vec![0; n]; n];
        for i in 0..n {
            for j in 0..n {
                a[i][j] = get();
            }
        }
        if p != 2 {
            println!("0");
            continue;
        }
        let mut row = vec![0; n];
        let mut col = vec![0; n];
        for i in 0..n {
            for j in 0..n {
                if a[i][j] == -1 {
                    row[i] += 1;
                    col[j] += 1;
                }
            }
        }
        if row.iter().any(|&v| v >= 2) || col.iter().any(|&v| v >= 2) {
            println!("0");
            continue;
        }
        let mut sub = vec![];
        for i in 0..n {
            if row[i] >= 1 { continue; }
            let mut tmp = 0i64;
            for j in 0..n {
                if col[j] == 0 {
                    if a[i][j] == 1 {
                        tmp |= 1 << j;
                    }
                }
            }
            sub.push(tmp);
        }
        let mut basis = vec![];
        for &(mut s) in &sub {
            for &b in &basis {
                s = min(s, s ^ b);
            }
            if s != 0 {
                basis.push(s);
            }
        }
        println!("{}", if basis.len() == sub.len() { 1 } else { 0 });
    }
}
