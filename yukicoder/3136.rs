fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

// Count how many B and F there are in hexadecimal representation of [0, x)
fn count_b_f(mut x: i64) -> i64 {
    let mut dp1 = [[0i64; 3]; 15];
    let mut dp2 = [[0i64; 3]; 15];
    let mut dig = vec![];
    for _ in 0..10 {
        dig.push((x % 16) as usize);
        x /= 16;
    }
    dp1[0][1] = 1;
    for d in dig {
        let mut ep1 = [[0; 3]; 15];
        let mut ep2 = [[0; 3]; 15];
        for r in 0..15 {
            for cmp in 0..3 {
                let val1 = dp1[r][cmp];
                let val2 = dp2[r][cmp];
                for next in 0..16 {
                    let nr = (r + next) % 15;
                    let ncmp = if d == next {
                        cmp
                    } else if d > next {
                        0
                    } else {
                        2
                    };
                    ep1[nr][ncmp] += val1;
                    ep2[nr][ncmp] += val2;
                    if next == 0xb || next == 0xf {
                        ep2[nr][ncmp] += val1;
                    }
                }
            }
        }
        dp1 = ep1;
        dp2 = ep2;
    }
    let mut ret = 0;
    for i in 0..15 {
        if i % 3 != 0 && i % 5 != 0 {
            ret += dp2[i][0];
        }
    }
    ret
}

fn len_fixlen(until: i64, len: i64) -> i64 {
    let hi = until - 1;
    let mut ret = (hi / 3) * 4;
    ret += (hi / 5) * 4;
    ret += (hi - hi / 5 - hi / 3 + hi / 15) * len;
    ret
}

fn len(until: i64) -> i64 {
    let mut ret = 0;
    let mut cur = 1;
    let mut dig = 1;
    while cur <= until {
        ret += len_fixlen(until.min(cur * 16), dig) - len_fixlen(cur, dig);
        dig += 1;
        cur *= 16;
    }
    ret
}

// S[0, n)
fn f(n: i64) -> i64 {
    let mut pass = 1;
    let mut fail = n + 2;
    while fail - pass > 1 {
        let mid = (pass + fail) / 2;
        if len(mid) <= n {
            pass = mid;
        } else {
            fail = mid;
        }
    }
    let mut ret = (pass - 1) / 5 + (pass - 1) / 3 + count_b_f(pass);
    let rem = (n - len(pass)) as usize;
    let cur_str;
    if pass % 15 == 0 {
        cur_str = vec![0xf, 0, 0, 0, 0xb, 0, 0, 0]
    } else if pass % 5 == 0 || pass % 3 == 0 {
        cur_str = vec![0xb, 0, 0, 0]
    } else {
        let mut x = pass;
        let mut digs = vec![];
        while x > 0 {
            let d = x % 16;
            digs.push(d);
            x /= 16;
        }
        digs.reverse();
        cur_str = digs;
    }
    for &d in &cur_str[..rem] {
        if d == 0xb || d == 0xf {
            ret += 1;
        }
    }
    ret
}

// https://yukicoder.me/problems/no/3136 (3.5)
// hex なのが誤読ポイントだった。
fn main() {
    let x: i64 = getline().trim().parse().unwrap();
    let y: i64 = getline().trim().parse().unwrap();
    println!("{}", f(y) - f(x - 1))
}
