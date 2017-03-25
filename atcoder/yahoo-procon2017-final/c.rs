#[allow(unused_imports)]
use std::cmp::*;
#[allow(unused_imports)]
use std::collections::*;
use std::io::Read;
#[allow(dead_code)]
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}
fn get_word() -> String {
    let mut stdin = std::io::stdin();
    let mut u8b: [u8; 1] = [0];
    loop {
        let mut buf: Vec<u8> = Vec::with_capacity(16);
        loop {
            let res = stdin.read(&mut u8b);
            if res.unwrap_or(0) == 0 || u8b[0] <= b' ' {
                break;
            } else {
                buf.push(u8b[0]);
            }
        }
        if buf.len() >= 1 {
            let ret = String::from_utf8(buf).unwrap();
            return ret;
        }
    }
}

#[allow(dead_code)]
fn get<T: std::str::FromStr>() -> T { get_word().parse().ok().unwrap() }

fn solve() {
    let n = get();
    let m: usize = get();
    let q: usize = get();
    let mut a: Vec<usize> = (0 .. n).map(|_| get::<usize>() % m).collect();
    const W: usize = 320;
    const N: usize = W * W;
    let mut dp = vec![vec![0; N]; W];
    let mut bias = vec![0; W];
    for i in 0 .. n {
        let b = i / W;
        dp[b][a[i]] += 1;
    }
    macro_rules! update {
        ($i:expr, $d:expr) => ({
            let olda = a[$i];
            let newa = (olda + $d) % m;
            let b = $i / W;
            dp[b][olda] -= 1;
            dp[b][newa] += 1;
            a[$i] = newa;

        });
    }
    for _ in 0 .. q {
        let l = get::<usize>() - 1;
        let r = get();
        let d = get::<usize>() % m;
        let lb = (l + W - 1) / W;
        let rb = r / W;
        let mut cnt = 0;
        if lb <= rb {
            for i in lb .. rb {
	        bias[i] += m - d;
	        bias[i] %= m;
            }
            for i in l .. lb * W {
                update!(i, d);
            }
            for i in rb * W .. r {
                update!(i, d);
            }
            for i in lb .. rb {
	        cnt += dp[i][bias[i]];
            }
            for i in l .. lb * W {
                if a[i] == bias[i / W] {
                    cnt += 1;
                }
            }
            for i in rb * W .. r {
                if a[i] == bias[i / W] {
                    cnt += 1;
                }
            }
        } else {
            for i in l .. r {
                update!(i, d);
            }
            for i in l .. r {
                if a[i] == bias[i / W] {
                    cnt += 1;
                }
            }
        }
        println!("{}", cnt);
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
