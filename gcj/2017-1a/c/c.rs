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
    let t: usize = get();
    for case_nr in 1 .. t + 1 {
        // for small only
        let hd: usize = get();
        let ad: usize = get();
        let hk: usize = get();
        let ak: usize = get();
        let b: usize = get();
        let d: usize = get();
        // possible ad values
        let mut ad_cand = vec![ad];
        if b > 0 {
            while let Some(&x) = ad_cand.last() {
                // No need to buff
                if x >= hk {
                    break;
                }
                ad_cand.push(x + b);
            }
        }
        const INF: i64 = 1 << 40;
        let mut dp = vec![vec![vec![vec![INF; ak + 1]; hk + 1]; ad_cand.len()]; hd + 1];
        let mut que = VecDeque::new();
        que.push_back((hd, 0, hk, ak, 0)); // ad_cand[0] == ad
        let mut minturn = INF;
        let orig_hd = hd;
        while let Some((hd, adi, hk, ak, turn)) = que.pop_front() {
            //println!("que: {:?}", (hd, adi, hk, ak, turn));
            if minturn <= turn { continue; }
            if hk == 0 {
                minturn = turn;
                continue;
            }
            let dist_ref = &mut dp[hd][adi][hk][ak];
            if *dist_ref <= turn {
                continue;
            }
            *dist_ref = turn;
            // attack
            let atk = ad_cand[adi];
            if hd > ak && hk > atk {
                que.push_back((hd - ak, adi, hk - atk,
                               ak, turn + 1));
            }
            if hk <= atk {
                que.push_back((hd, adi, 0,
                               ak, turn + 1));
            }
            // buff
            if hd > ak && adi < ad_cand.len() - 1 {
                que.push_back((hd - ak, adi + 1, hk,
                               ak, turn + 1));
            }
            // cure
            if orig_hd > ak {
                que.push_back((orig_hd - ak, adi, hk,
                               ak, turn + 1));
            }
            // debuff
            if d > 0 {
                let deb = if ak > d { ak - d } else { 0 };
                if hd > deb {
                    que.push_back((hd - deb, adi, hk,
                                   deb, turn + 1));
                }
            }
        }
        println!("Case #{}: {}", case_nr, if minturn == INF { "IMPOSSIBLE".to_string() }
                 else { format!("{}", minturn) });
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
