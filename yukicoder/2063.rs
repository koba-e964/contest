use std::cmp::*;

fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let n: Vec<_> = getline().trim().chars().collect();
    let mut mi = n.len();
    let mut ma = 0;
    let mut c = 0;
    for i in 0..n.len() {
        if n[i] == '1' {
            mi = min(mi, i);
            ma = max(ma, i);
            c += 1;
        }
    }
    if mi != ma && (c == 2 || c == ma - mi + 1) {
        println!("Yes");
    } else {
        println!("No");
    }
}
