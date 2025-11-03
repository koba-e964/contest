fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

// Tags: dp, experiments, grids-as-bipartite-graphs
fn main() {
    let ints = getline().trim().split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let [n, q] = ints[..] else { panic!() };
    let mut dp = vec![vec![vec![false; n * n + 1]; n + 1]; n + 1];
    dp[0][0][0] = true;
    for i in 0..n {
        for j in 0..n {
            for k in 0..n * n {
                if !dp[i][j][k] { continue; }
                for a in 2..=n - i {
                    for b in 2..=n - j {
                        dp[i + a][j + b][k + a * b] = true;
                    }
                }
            }
        }
    }
    let mut ans = vec![false; n * n + 1];
    for i in 0..=n {
        for j in 0..=n {
            for k in 0..=n * n {
                if dp[i][j][k] {
                    ans[k] = true;
                }
            }
        }
    }
    for _ in 0..q {
        let k = getline().trim().parse::<usize>().unwrap();
        println!("{}", if ans[n * n - k] {
            "Yes"
        } else {
            "No"
        });
    }
}
