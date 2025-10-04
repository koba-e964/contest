fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

const INF: i64 = 1 << 58;

// Tags: interval-dp
fn main() {
    getline();
    let a = getline().trim().split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let n = a.len();
    let mut c = vec![0; n + 1];
    for i in 0..n {
        c[i + 1] = c[i] ^ a[i];
    }
    let mut dp = vec![vec![[INF; 2]; n + 1]; n];
    for i in 0..n {
        dp[i][i + 1] = [INF, 0];
    }
    for s in 2..n + 1 {
        for i in 0..n - s + 1 {
            let j = i + s;
            let mut me = [INF; 2];
            for k in i + 1..j {
                let sub1 = dp[i][k];
                let sub2 = dp[k][j];
                for a1 in 0..2 {
                    let b1 = if (k + j) % 2 == 0 {
                        sub1[a1]
                    } else {
                        if a1 == 1 {
                            INF
                        } else {
                            sub1[0].min((c[i] ^ c[j]) + sub1[1])
                        }
                    };
                    if b1 >= INF { continue; }
                    for a2 in 0..2 {
                        let b2 = if (i + k) % 2 == 0 {
                            sub2[a2]
                        } else {
                            if a2 == 1 {
                                INF
                            } else {
                                sub2[0].min((c[i] ^ c[j]) + sub2[1])
                            }
                        };
                        if b2 >= INF { continue; }
                        me[a1 + a2] = me[a1 + a2].min(b1 + b2);
                    }
                }
            }
            dp[i][j] = me;
            if n <= 8 {
                eprintln!("dp[{i}][{j}] = {:?}", me);
            }
        }
    }
    let ans = dp[0][n];
    println!("{}", ans[0].min(ans[1] + c[n]));
}
