fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let s: Vec<char> = getline().trim().chars().collect();
    let t: Vec<char> = "atcoder".chars().collect();
    let mut v = vec![0; 7];
    for i in 0..7 {
        let x = t.iter().position(|&c| c == s[i]).unwrap();
        v[i] = x;
    }
    let mut ans = 0;
    for i in 0..7 {
        for j in 0..i {
            if v[j] > v[i] { ans += 1; }
        }
    }
    println!("{}", ans);
}
