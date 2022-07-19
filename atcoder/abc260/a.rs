use std::collections::*;

fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let s = getline().trim().to_string();
    let mut hm = HashMap::new();
    for c in s.chars() {
        *hm.entry(c).or_insert(0) += 1;
    }
    for (k, v) in hm {
        if v == 1 {
            println!("{}", k);
            return;
        }
    }
    println!("-1");
}
