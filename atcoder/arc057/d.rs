#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::*;
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok();
    return ret;
}
fn get_word() -> String {
    let mut stdin = std::io::stdin();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.is_err() || res.ok().unwrap() == 0 || u8b[0] <= ' ' as u8 {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = std::string::String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}
fn parse<T: std::str::FromStr>(s: &str) -> T { s.parse::<T>().ok().unwrap() }

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { parse(&get_word()) }

const MOD: i64 = 1_000_000_007;

fn solve(dp: &mut HashMap<(Vec<usize>, usize), i64>, a: &HashSet<usize>, v: &Vec<usize>, e: usize, n: usize) -> i64 {
    if let Some(&res) = dp.get(&(v.clone(), e)) {
        return res;
    }
    if e == n * (n - 1) / 2 {
        return 1;
    }
    let m = v.len();
    let mut sum: i64 = 0;
    if a.contains(&e) {
        for i in 0 .. m {
            for j in i + 1 .. m {
            // unify v[i] and v[j]
                let mut q = Vec::new();
                for k in 0 .. m {
                    if k != i && k != j {
                        q.push(v[k]);
                    }
                }
                q.push(v[i] + v[j]);
                q.sort();
                let fact = (v[i] as i64) * (v[j] as i64) % MOD;
                sum += solve(dp, a, &q, e + 1, n) * fact % MOD;
                sum %= MOD;
            }
        }
    } else {
        let mut rem: i64 = 0;
        for cc in v {
            rem += (cc * (cc - 1) / 2) as i64;
        }
        rem -= e as i64;
        sum = rem * solve(dp, a, v, e + 1, n) % MOD;
    }
    dp.insert((v.clone(), e), sum);
    return sum;
}

fn main() {
    let n: usize = get();
    let a: HashSet<usize> = (0..n - 1).map(|_| get::<usize>() - 1).collect();
    let mut dp: HashMap<(Vec<usize>, usize), i64> = HashMap::new();
    println!("{}", solve(&mut dp, &a, &vec![1; n], 0, n));
}
