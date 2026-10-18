fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let m = ints[1];
    let a = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let n = a.len();
    let s = a.iter().sum::<i64>();
    let piece = s / m;
    let mut dp = vec![0; 1 << n];
    for bits in 1..1 << n {
        let mut me = 0;
        let mut sum = 0;
        for i in 0..n {
            if (bits & 1 << i) != 0 {
                sum += a[i];
            }
        }
        for i in 0..n {
            if (bits & 1 << i) == 0 {
                continue;
            }
            me = me.max(dp[bits ^ 1 << i]);
        }
        let val = if sum % piece == 0 {
            1
        } else {
            0
        };
        dp[bits] = me + val;
    }
    println!("{}", m - dp[(1 << n) - 1]);
}
