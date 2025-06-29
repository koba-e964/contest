fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn amod(a: i32, b: i32, p: i32) -> i32 {
    let mut r = a + b;
    if r >= p {
        r -= p;
    }
    r
}

fn powmod(x: i64, mut e: i64, m: i64) -> i64 {
    let mut sum = 1;
    let mut cur = x % m;
    while e > 0 {
        if e % 2 != 0 {
            sum = sum * cur % m;
        }
        cur = cur * cur % m;
        e /= 2;
    }
    sum
}

// https://yukicoder.me/problems/no/3187 (3)
// TLE したので定数倍高速化 (変数による除算を減らす) をした。
// -> まだ TLE なので、配列の型を i64 から i32 に変えた。
// Tags: sqrt-decomposition, divisors, grouping-by-quotients
fn main() {
    let ints = getline()
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let n = ints[0] as usize;
    let p = ints[1];
    let pp = p as i32;
    let mut dp = vec![0; n + 1];
    let mut divs = vec![vec![]; n + 1];
    for i in 1..=n {
        for j in (i..=n).step_by(i) {
            divs[j].push(i);
        }
    }
    let mut mul_acc = vec![vec![]; n + 1];
    for i in 1..=n {
        mul_acc[i] = vec![0; n / i + 2];
    }
    for i in 3..=n {
        let mut loopback = 0;

        let mut sq = 0;
        let mut me = 0;
        // grouping by quotients
        while sq * sq <= i as u32 {
            sq += 1;
        }
        sq -= 1;
        for b in 1..=sq {
            let to = i as u32 / b * b;
            if to == i as u32 {
                loopback += 1;
            } else {
                me = amod(me, dp[to as usize], pp);
            }
        }
        let mut old = i as u32;
        for d in 1..=sq {
            let mut r = old;
            old = i as u32 / (d + 1);
            let l = sq.max(old);
            // (l, r]
            if l >= r {
                continue;
            }
            if r * d == i as u32 {
                loopback += 1;
                r -= 1;
            }
            let dd = d as usize;
            me = amod(me, mul_acc[dd][r as usize + 1], pp);
            me = amod(me, pp - mul_acc[dd][l as usize+ 1], pp);
        }

        // (me + i) / non_loopback
        me = amod(me, i as i32, pp);
        dp[i] = (me as i64 * powmod(i as i64 - loopback, p - 2, p) % p) as i32;
        for &d in &divs[i] {
            let j = i / d;
            mul_acc[d][j + 1] = amod(mul_acc[d][j], dp[i], pp);
        }
    }
    println!("{}", dp[n]);
}
