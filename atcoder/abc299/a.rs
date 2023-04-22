fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    getline();
    let s: Vec<_> = getline().trim().chars().collect();
    let mut v = vec![];
    let mut c = 0;
    for i in 0..s.len() {
        if s[i] == '*' {
            c = i;
        }
        if s[i] == '|' {
            v.push(i);
        }
    }
    println!("{}", if v[0] < c && c < v[1] { "in" } else { "out" });
}
