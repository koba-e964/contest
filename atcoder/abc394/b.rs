fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let n: usize = getline().trim().parse().unwrap();
    let mut s = vec![];
    for _ in 0..n {
        s.push(getline().trim().to_string());
    }
    s.sort_by_key(|s| s.len());
    let mut ans = "".to_string();
    for s in s {
        ans += &s;
    }
    println!("{ans}");
}
