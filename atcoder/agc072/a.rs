fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

const INF: i64 = 1 << 60;

fn solve(abc: &[(i64, i64, i64)]) -> bool {
    eprintln!("abc = {abc:?}");
    let n = abc.len();
    let mut csum = vec![0; n + 1];
    for i in 0..n {
        csum[i + 1] = csum[i] + abc[i].2;
    }
    let mut dp = vec![INF; n + 1];
    dp[0] = 0;
    for i in 0..n {
        let mut mi = INF;
        for j in i + 1..n + 1 {
            let mut t = dp[i].max(abc[j - 1].0);
            t += abc[j - 1].2;
            // t + csum[k] - csum[i] <= abc[k].1 - abc[k].2 for all i <= k < j-1
            if t <= mi && t <= abc[j - 1].1 {
                let t = t + csum[j - 1] - csum[i];
                dp[j] = dp[j].min(t);
            }
            let k = j - 1;
            mi = mi.min(abc[k].1 - abc[k].2 - csum[k] + csum[i]);
        }
    }
    dp[n] <= abc[n - 1].1
}

// The author read the editorial before implementing this.
// Tags: permutations, operations, swap-argument, scheduling, intervals
fn main() {
    let t: i32 = getline().trim().parse().unwrap();
    for _ in 0..t {
        let n = getline().trim().parse::<usize>().unwrap();
        let d = getline().trim().parse::<i64>().unwrap();
        let mut tx = vec![];
        for _ in 0..n {
            let ints = getline().trim().split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            let [t, x] = ints[..] else { panic!() };
            tx.push((0.max(t - x), t + x + d, 2 * x));
        }
        tx.sort_by_key(|u| u.1);
        println!("{}", if solve(&tx) {
            "Yes"
        } else {
            "No"
        });
    }
}
