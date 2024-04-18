fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let mut s = getline().trim().bytes().collect::<Vec<_>>();
    let n = s.len();
    for i in 0..n {
        if (0..n).all(|j| i == j || s[j] != s[i]) {
            println!("{}", i + 1);
            return;
        }
    }
}
