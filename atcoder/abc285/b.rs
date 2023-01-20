fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    getline();
    let s: Vec<_> = getline().trim().chars().collect();
    let n = s.len();
    for i in 1..n {
        let mut l = n - i;
        for j in 0..n - i {
            if s[j] == s[j + i] {
                l = j;
                break;
            }
        }
        println!("{}", l);
    }
}
