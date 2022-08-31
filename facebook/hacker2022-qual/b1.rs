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

fn solve(a: &[Vec<char>]) -> Option<Vec<Vec<char>>> {
    let n = a.len();
    let m = a[0].len();
    let mut num = 0;
    for i in 0..n {
        for j in 0..m {
            if a[i][j] == '^' {
                num += 1;
            }
        }
    }
    if num == 0 {
        return Some(a.to_vec());
    }
    if n == 1 || m == 1 {
        return None;
    }
    Some(vec![vec!['^'; m]; n])
}

fn main() {
    let t: usize = get();
    for case_nr in 1..t + 1 {
        let r: usize = get();
        let _c: usize = get();
        let a: Vec<Vec<char>> = (0..r).map(|_| get_word().chars().collect()).collect();
        if let Some(sol) = solve(&a) {
            println!("Case #{}: Possible", case_nr);
            for i in 0..r {
                println!("{}", sol[i].iter().cloned().collect::<String>());
            }
        } else {
            println!("Case #{}: Impossible", case_nr);
        }
    }
}
