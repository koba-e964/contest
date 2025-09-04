fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

// https://yukicoder.me/problems/no/2222 (3.5)
// dp[i] := i でセーブした状態で、そこからの期待値 とする。
// 100 マスくらい進む前にはほぼ確実にセーブされるので、それぞれの地点で最大 100 マス DP すれば良い。
fn main() {
    getline();
    let s = getline().trim().chars().collect::<Vec<_>>();
    let n = s.len();
    let mut dp = vec![0.0; n];
    const D: usize = 100;
    for i in (0..n - 1).rev() {
        if s[i] == '#' {
            dp[i] = 1.0 / 0.0;
            continue;
        }
        let l = D.min(n - 1 - i);
        let mut aux = vec![0.0; l];
        let mut aux_l = vec![0.0; l];
        for j in (0..l).rev() {
            if s[i + j] == '#' {
                aux[j] = 0.0;
                aux_l[j] = 1.0;
                continue;
            }
            let mut loopback = 0.0;
            let mut sum = 0.0;
            for step in 1..=2 {
                if i + j + step >= n {
                    loopback += 1.0;
                } else if i + j + step < n - 1 {
                    sum += if j + step < l { aux[j + step] } else { 0.0 };
                    loopback += if j + step < l { aux_l[j + step] } else { 0.0 };
                }
            }
            // save
            if j > 0 {
                sum += dp[i + j];
            } else {
                loopback += 1.0;
            }
            aux[j] = sum / 3.0 + 1.0;
            aux_l[j] = loopback / 3.0;
        }
        dp[i] = aux[0] / (1.0 - aux_l[0]);
    }
    println!("{}", dp[0]);
}
