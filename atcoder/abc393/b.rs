fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let s = getline().trim().chars().collect::<Vec<_>>();
    let n = s.len();
    let mut ans = 0;
    for i in 0..n {
        if s[i] != 'A' {
            continue;
        }
        for k in i + 2..n {
            if s[k] != 'C' {
                continue;
            }
            if (i + k) % 2 == 0 {
                if s[(i + k) / 2] == 'B' {
                    ans += 1;
                }
            }
        }
    }
    println!("{ans}");
}
