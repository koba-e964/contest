fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let t: i32 = getline().trim().parse().unwrap();
    for _ in 0..t {
        getline();
        let s = getline().trim().chars().collect::<Vec<_>>();
        let mut n = 1;
        while (1 << n) - 1 < s.len() {
            n += 1;
        }
        let mut dp = vec![false; 1 << n];
        dp[0] = true;
        for bits in 1..1 << n {
            if s[bits - 1] == '1' {
                continue;
            }
            let mut ok = false;
            for i in 0..n {
                if bits & (1 << i) == 0 {
                    continue;
                }
                if dp[bits ^ (1 << i)] {
                    ok = true;
                }
            }
            dp[bits] = ok;
        }
        if dp[(1 << n) - 1] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
