use std::collections::*;
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

fn naive(s: &[char]) -> i64 {
    let n = s.len();
    let mut a = 0u64;
    for i in 0..n {
        if s[i] == '1' {
            a |= 1 << i;
        }
    }
    // Swapping with length 2 is sufficient.
    let mut vis = HashSet::new();
    let mut que = VecDeque::new();
    que.push_back(a);
    while let Some(v) = que.pop_front() {
        for i in 0..n - 1 {
            if ((v >> i) & 3) != 1 { continue; }
            for j in 0..n - 1 {
                if ((v >> j) & 3) != 2 { continue; }
                if (i as i32 - j as i32).abs() <= 1 { continue; }
                let sw = v ^ 3 << i ^ 3 << j;
                if vis.contains(&sw) { continue; }
                vis.insert(sw);
                que.push_back(sw);
            }
        }
    }
    
    let mut vis: Vec<_> = vis.into_iter().collect();
    vis.sort_by_key(|&x| x.reverse_bits());
    for &v in &vis {
        for j in 0..n {
            eprint!("{}", (v >> j) & 1);
        }
        eprintln!();
    }
    vis.len() as i64
}

fn main() {
    let s: Vec<_> = get_word().chars().collect();
    let n = s.len();
    let mut dp = vec![vec![vec![0i64; n * (n - 1) / 2 + 1]; n + 1]; n + 1];
    dp[0][0][0] = 1;
    for i in 0..n {
        let lim = i * (i + 1) / 2 - i;
        for j in 0..i + 1 {
            for k in 0..lim + 1 {
                let val = dp[i][j][k];
                dp[i + 1][j][k] += val;
                dp[i + 1][j + 1][k + i] += val;
            }
        }
    }
    let mut one = 0;
    let mut sum = 0;
    for i in 0..n {
        if s[i] == '1' {
            one += 1;
            sum += i;
        }
    }
    println!("{}", dp[n][one][sum]);
    if n <= 10 {
        eprintln!("naive = {}", naive(&s));
    }
}
