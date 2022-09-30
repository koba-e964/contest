fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

const MOD: i32 = 10_000;

fn add(a: &mut i32, b: i32) {
    *a += b;
    if *a >= MOD {
        *a -= MOD;
    }
}

fn calc(a: &[i32], m: usize) -> i32 {
    if a.is_empty() { return 0; }
    let n = a.len();
    let mut dp = vec![vec![[[[0; 10]; 2]; 2]; m]; n];
    for i in 0..n {
        for inc in 0..2 {
            for dig in 1..if i == 0 { a[i] + 1 } else { 10 } {
                add(&mut dp[i][dig as usize % m][inc][if i == 0 && dig == a[0] { 1 } else { 0 }][dig as usize], 1);
            }
            if i > 0 {
                for j in 0..m {
                    for eq in 0..2 {
                        for dig in 0..if eq == 0 { 10 } else { a[i] + 1 } {
                            for last in if inc == 0 { 0..dig } else { dig + 1..10 } {
                                let pre = dp[i - 1][j][1 - inc][eq][last as usize];
                                add(&mut dp[i][(10 * j + dig as usize) % m][inc][eq & if dig == a[i] { 1 } else { 0 }][dig as usize], pre);
                            }
                        }
                    }
                }
            }
        }
    }
    let mut ans = 0;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..10 {
                add(&mut ans, dp[n - 1][0][i][j][k]);
            }
        }
    }
    for i in 1..if n >= 2 { 10 } else { a[0] as usize + 1 } {
        if i % m == 0 {
            add(&mut ans, MOD - 1);
        }
    }
    ans
}

// Tags: digital-dp, multi-precision-integers
fn main() {
    let mut a: Vec<_> = getline().trim().bytes().map(|c| (c - b'0') as i32).collect();
    let b: Vec<_> = getline().trim().bytes().map(|c| (c - b'0') as i32).collect();
    let m: usize = getline().trim().parse().unwrap();
    let mut borrow = 1;
    for i in (0..a.len()).rev() {
        a[i] -= borrow;
        borrow = 0;
        if a[i] < 0 {
            a[i] += 10;
            borrow = 1;
        }
    }
    if a[0] == 0 {
        a.remove(0);
    }
    println!("{}", (calc(&b, m) + MOD - calc(&a, m)) % MOD);
}
