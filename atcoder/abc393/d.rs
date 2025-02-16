fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    getline();
    let s = getline().trim().chars().collect::<Vec<_>>();
    let n = s.len();
    let mut idx = vec![];
    for i in 0..n {
        if s[i] == '1' {
            idx.push(i);
        }
    }
    let cent = idx[idx.len() / 2];
    let mut ans = 0i64;
    for i in 0..idx.len() {
        let targ = cent - idx.len() / 2 + i;
        ans += (idx[i] as i64 - targ as i64).abs();
    }
    println!("{ans}");
}
