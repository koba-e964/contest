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
    let h: usize = get();
    let w: usize = get();
    let d: i32 = get();
    let mut s = vec![];
    for _ in 0..h {
        s.push(get_word().chars().collect::<Vec<_>>());
    }
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }
            for a in 0..h {
                for b in 0..w {
                    if s[a][b] == '#' {
                        continue;
                    }
                    let mut cnt = 0;
                    for k in 0..h {
                        (0..w).filter(|&l| s[k][l] == '.').for_each(|l| {
                            let d1 = (i as i32 - k as i32).abs() + (j as i32 - l as i32).abs();
                            let d2 = (a as i32 - k as i32).abs() + (b as i32 - l as i32).abs();
                            if d1.min(d2) <= d {
                                cnt += 1;
                            }
                        });
                    }
                    ans = ans.max(cnt);
                }
            }
        }
    }
    println!("{ans}");
}
