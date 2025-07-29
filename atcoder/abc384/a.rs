fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let vals = getline().split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let a = vals[1].chars().next().unwrap();
    let b = vals[2].chars().next().unwrap();
    let s = getline().trim().chars()
        .map(|c| if c == a { a } else { b })
        .collect::<String>();
    println!("{s}");
}
