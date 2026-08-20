fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn calc(a: Vec<i32>, p: f64) -> Vec<Vec<f64>> {
    let n = a.len();
    let mut dp = vec![0.0f64; 1 << n];
    dp[0] = 1.0;
    let mut ret = vec![vec![0.0; n]; n];
    for bits in 0usize..(1 << n) - 1 {
        let mut distr = vec![0.0; n];
        let bc = n - bits.count_ones() as usize;
        if bc == 1 {
            for i in 0..n {
                if (bits & 1 << i) == 0 {
                    distr[i] = 1.0;
                }
            }
        } else {
            let rem = (1.0 - p) / (bc - 1) as f64;
            let mut first = true;
            for i in 0..n {
                if (bits & 1 << i) != 0 {
                    continue;
                }
                if first {
                    distr[i] = p;
                } else {
                    distr[i] = rem;
                }
                first = false;
            }
        }
        let me = dp[bits];
        for i in 0..n {
            if distr[i] != 0.0 {
                let val = me * distr[i];
                dp[bits ^ 1 << i] += val;
                ret[n - bc][i] += val;
            }
        }
    }
    ret
}

// https://yukicoder.me/problems/no/174 (4)
// 求めるものは期待値なので、各試合での得点の期待値の和が答えである。
// AもBも出すカードは相手に依存せずに決めるので、それぞれが各試合で出すカードの確率が計算できる (O(2^N))。
// そのtableを使って計算すればOK。
// - (input)
// - bitDPでカードの確率を計算する (output: c[N][N], d[N][N])
//   - 2^n loop を書く
//   - transの確率分布を計算する
// - 期待値を計算する -> ans
// - ans をprintlnする
fn main() {
    let fs = getline().trim().split_whitespace()
        .map(|x| x.parse::<f64>().unwrap())
        .collect::<Vec<_>>();
    let n = fs[0] as usize;
    let pa = fs[1];
    let pb = fs[2];
    let mut a = getline().trim().split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let mut b = getline().trim().split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    a.sort();
    b.sort();
    let c = calc(a.clone(), pa);
    let d = calc(b.clone(), pb);
    let mut ans = 0.0;
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if a[j] > b[k] {
                    ans += c[i][j] * d[i][k] * (a[j] + b[k]) as f64;
                }
            }
        }
    }
    println!("{ans}");
}
