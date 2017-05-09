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


// Solution in the editorial
fn solve() {
    let mut n: Vec<i32> = get_word().bytes().map(|x| (x - b'0') as i32)
        .collect();
    n.reverse();
    let l = n.len();
    let mut sum = 0;
    // calculate 9n
    {
        let mut carry = 0;
        for i in 0 .. l {
            let tmp = n[i] * 9 + carry;
            let nc = tmp / 10;
            let tmp = tmp % 10;
            n[i] = tmp;
            carry = nc;
        }
        if carry > 0 {
            n.push(carry);
        }
    }
    for i in 0 .. l {
        sum += n[i];
    }
    for k in 0 .. l + 1 {
        if sum as usize <= 9 * k {
            println!("{}", k);
            return;
        }
        // add 9 to n, taking care of overflow
        let mut carry = 9;
        for i in 0 .. l {
            sum -= n[i];
            let tmp = n[i] + carry;
            let nc = tmp / 10;
            let tmp = tmp % 10;
            n[i] = tmp;
            carry = nc;
            // adjusting sum
            sum += n[i];
            if carry == 0 {
                // By this pruning, amortized computation time becomes O(1)
                break;
            }
        }
        if carry > 0 {
            n.push(carry);
            sum += carry;
        }
    }
}

fn main() {
    // In order to avoid potential stack overflow, spawn a new thread.
    let stack_size = 104_857_600; // 100 MB
    let thd = std::thread::Builder::new().stack_size(stack_size);
    thd.spawn(|| solve()).unwrap().join().unwrap();
}
