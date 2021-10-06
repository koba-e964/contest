use std::cmp::*;
use std::collections::*;

fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let s: Vec<_> = getline().trim().chars().collect();
    let n = s.len();
    let mut hm = HashMap::new();
    hm.insert([0; 4], 0);
    let t = ['K', 'U', 'R', 'O', 'I'];
    for i in 0..n {
        let mut nxt = vec![];
        for j in 0..5 {
            if s[i] != t[j] && s[i] != '?' { continue; }
            for (&k, &v) in &hm {
                if j >= 1 && k[j - 1] == 0 { continue; }
                let mut nxtk = k;
                if j >= 1 {
                    nxtk[j - 1] -= 1;
                }
                let mut nxtv = v;
                if j <= 3 {
                    nxtk[j] += 1;
                } else {
                    nxtv += 1;
                }
                nxt.push((nxtk, nxtv));
            }
        }
        for (k, v) in nxt {
            let ent = hm.entry(k).or_insert(0);
            *ent = max(*ent, v);
        }
    }
    println!("{}", hm.values().max().unwrap());
}
