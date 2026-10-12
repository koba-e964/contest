use std::collections::*;
fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn calc(x: i64, a: &[i64]) -> i64 {
    let mut que = BinaryHeap::new();
    que.push((x, 1));
    for &a in a {
        let mut acnt = 0;
        loop {
            if let Some((x, rep)) = que.pop() {
                if x < a {
                    que.push((x, rep));
                    break;
                }
                let q = x / a;
                let r = x - q * a;
                if r > 0 {
                    que.push((r, rep));
                }
                acnt += q * rep;
            } else {
                break;
            }
        }
        if acnt > 0 {
            que.push((a, acnt));
        }
    }
    let mut ans = 0;
    for (_, c) in que {
        ans += c;
    }
    ans
}

fn main() {
    let t = getline().trim().parse::<i32>().unwrap();
    for _ in 0..t {
        let ints = getline().trim().split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let x = ints[1];
        let a = getline().trim().split_whitespace()
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        println!("{}", calc(x + 1, &a) - 1);
    }
}
