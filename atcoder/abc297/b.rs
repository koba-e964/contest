fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn is_chess960(s: &[char]) -> bool {
    assert_eq!(s.len(), 8);
    let mut bs = vec![];
    let mut k = 0;
    let mut rs = vec![];
    for i in 0..8 {
        match s[i] {
            'B' => bs.push(i),
            'K' => k = i,
            'R' => rs.push(i),
            _ => {}
        }
    }
    if (bs[0] + bs[1]) % 2 == 0 {
        return false;
    }
    rs[0] < k && k < rs[1]
}

// Tags: chess
fn main() {
    let s: Vec<char> = getline().chars().collect();
    println!("{}", if is_chess960(&s[..8]) { "Yes" } else { "No" });
}
