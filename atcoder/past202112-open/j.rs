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

// Tags: matrix, rotation, implementation
fn main() {
    let n: usize = get();
    let q: usize = get();
    let mut tbl = vec![vec![0; n]; n];
    let mut g = [[1, 0, 0], [0, 1, 0]];
    for _ in 0..q {
        let ty: i32 = get();
        match ty {
            1 => {
                let x = get::<isize>() - 1;
                let y = get::<isize>() - 1;
                let nx = if g[0][0] != 0 {
                    (x - g[0][2]) / g[0][0]
                } else {
                    (y - g[1][2]) / g[1][0]
                };
                let ny = if g[1][1] != 0 {
                    (y - g[1][2]) / g[1][1]
                } else {
                    (x - g[0][2]) / g[0][1]
                };
                tbl[nx as usize][ny as usize] ^= 1;
            }
            2 => {
                let k: String = get();
                g.swap(0, 1);
                if k == "B" {
                    for j in 0..3 {
                        g[0][j] *= -1;
                    }
                    g[0][2] += n as isize - 1;
                } else {
                    for j in 0..3 {
                        g[1][j] *= -1;
                    }
                    g[1][2] += n as isize - 1;
                }
            }
            3 => {
                let k: String = get();
                if k == "A" {
                    // x -> n - 1 - x
                    for j in 0..3 {
                        g[0][j] *= -1;
                    }
                    g[0][2] += n as isize - 1;
                } else {
                    for j in 0..3 {
                        g[1][j] *= -1;
                    }
                    g[1][2] += n as isize - 1;
                }
            }
            _ => panic!(),
        }
    }
    let mut ans = vec![vec![0; n]; n];
    for i in 0..n as isize {
        for j in 0..n as isize {
            let x = g[0][0] * i + g[0][1] * j + g[0][2];
            let y = g[1][0] * i + g[1][1] * j + g[1][2];
            ans[x as usize][y as usize] = tbl[i as usize][j as usize];
        }
    }
    for i in 0..n {
        for j in 0..n {
            print!("{}", ans[i][j]);
        }
        println!();
    }
}
