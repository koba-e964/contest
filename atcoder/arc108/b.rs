fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    getline();
    let mut v = vec![];
    for c in getline().trim().chars() {
        v.push(c);
        if v.len() >= 3 && v[v.len() - 3..] == ['f', 'o', 'x'] {
            v.truncate(v.len() - 3);
        }
    }
    println!("{}", v.len());
}
