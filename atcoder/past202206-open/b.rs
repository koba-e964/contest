use std::cmp::*;
use std::collections::*;

fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let s: Vec<_> = getline().trim().chars().collect();
    let mut hm = HashMap::new();
    for c in s.windows(2) {
        *hm.entry(c).or_insert(0) += 1;
    }
    let mut mi = (0, &[] as &[_]);
    for (k, v) in hm {
        mi = min(mi, (-v, k));
    }
    println!("{}", mi.1.iter().cloned().collect::<String>());
}
