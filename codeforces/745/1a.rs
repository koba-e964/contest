use std::cmp::*;
use std::io::{Write, BufWriter, Read};

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

const INF: i64 = 1 << 50;

fn main() {
    let debug = false;
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());
    macro_rules! puts {($($format:tt)*) => (let _ = write!(out,$($format)*););}
    let t: usize = get();
    for _ in 0..t {
        let n: usize = get();
        let m: usize = get();
        let mut s = vec![];
        for _ in 0..n {
            let t: Vec<_> = get_word().bytes().collect();
            s.push(t);
        }
        let mut acc1 = vec![vec![0; m]; n + 1];
        for i in 0..n {
            for j in 0..m {
                acc1[i + 1][j] = acc1[i][j]
                    + if s[i][j] == b'1' { 1 } else { 0 };
            }
        }
        let mut rows = vec![vec![0; m + 1]; 3];
        let mut ans = INF;
        for i in 0..n {
            for j in i + 4..n {
                for k in 0..m {
                    rows[0][k + 1] = rows[0][k] + acc1[i + 1][k] - acc1[i][k];
                    rows[1][k + 1] = rows[1][k] + acc1[j][k] - acc1[i + 1][k];
                    rows[2][k + 1] = rows[2][k] + acc1[j + 1][k] - acc1[j][k];
                }
                if debug {
                    eprintln!("rows = {:?}", rows);
                }
                let mut ma = (j - i - 1) as i64
                    - rows[1][1] + rows[0][1] + rows[2][1] - rows[1][1];
                for k in 3..m {
                    let right = (j - i - 1) as i64 - rows[1][k + 1] + rows[1][k];
                    let val = rows[1][k] + 2 * (k as i64 - 1)
                        - rows[2][k] - rows[0][k]
                        + right;
                    ans = min(ans, val + ma);
                    if debug {
                        eprintln!("[{} {}] {} => {} (right = {})", i, j, k, val + ma, right);
                    }
                    let t = k - 2;
                    ma = min(ma, (j - i - 1) as i64
                             - rows[1][t + 1] + rows[0][t + 1] + rows[2][t + 1]
                             - (rows[1][t + 1] - rows[1][t]) - 2 * t as i64);
                }
            }
        }
        puts!("{}\n", ans);
    }
}
