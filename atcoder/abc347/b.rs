use std::collections::*;

fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let s = getline().trim().bytes().collect::<Vec<_>>();
    let mut hs = HashSet::new();
    for i in 0..s.len() + 1 {
        for j in 0..i {
            hs.insert(s[j..i].to_vec());
        }
    }
    println!("{}", hs.len());
}
