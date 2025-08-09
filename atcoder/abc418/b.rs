fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let s = getline().trim().chars().collect::<Vec<_>>();
    let mut ans = 0.0f64;
    let n = s.len();
    for i in 0..n {
        for j in i + 2..n {
            if s[i] != 't' || s[j] != 't' {
                continue;
            }
            let mut tmp = 0.0;
            for k in i + 1..j {
                if s[k] == 't' {
                    tmp += 1.0;
                }
            }
            tmp /= (j - i - 1) as f64;
            ans = ans.max(tmp);
        }
    }
    println!("{ans}");
}
