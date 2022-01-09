fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let s: Vec<_> = getline().trim().chars().collect();
    let n = s.len();
    let mut ans = 500;
    let hand = |x: char| {
        '1' <= x && x <= '5'
    };
    for i in 1..n {
        ans += match (s[i - 1] == s[i], hand(s[i - 1]) == hand(s[i])) {
            (true, _) => 301,
            (false, true) => 210,
            _ => 100,
        };
    }
    println!("{}", ans);
}
