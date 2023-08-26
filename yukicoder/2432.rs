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

// https://yukicoder.me/problems/no/2432 (3)
// 2HW 回操作すれば必ず (1,1) に戻るので、 4HW 回の操作で同じマスを偶数回踏み初期状態に戻る。
fn main() {
    let h: usize = get();
    let w: usize = get();
    let k: usize = get();
    let k = k % (4 * h * w);
    let mut ans = vec![vec![false; w]; h];
    for i in 0..k {
        let mut x = i % (2 * h);
        if x >= h {
            x = 2 * h - 1 - x;
        }
        let mut y = i % (2 * w);
        if y >= w {
            y = 2 * w - 1 - y;
        }
        ans[x][y] ^= true;
    }
    for i in 0..h {
        let mut s = "".to_string();
        for j in 0..w {
            if ans[i][j] {
                s.push('#');
            } else {
                s.push('.');
            }
        }
        println!("{}", s);
    }
}
