fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let s = getline().trim().chars().collect::<Vec<_>>();
    let n = s.len();
    println!("{}", if s[0] == '<' && s[n - 1] == '>' && s[1..n - 1].iter().all(|&c| c == '=') {
        "Yes"
    } else {
        "No"
    });
}
