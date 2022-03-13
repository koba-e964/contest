fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let mut s: Vec<_> = getline().trim().chars().collect();
    s.sort();
    println!("{}", s.into_iter().collect::<String>());
}
