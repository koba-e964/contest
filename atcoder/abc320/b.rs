fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let s = getline().trim().chars().collect::<Vec<_>>();
    let n = s.len();
    let mut ans = 1;
    for i in 0..n {
        for j in i + 2..n + 1 {
            let mut x = s[i..j].to_vec();
            x.reverse();
            if s[i..j] == x {
                ans = std::cmp::max(ans, j - i);
            }
        }
    }
    println!("{}", ans);
}
