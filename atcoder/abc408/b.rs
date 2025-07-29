fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    getline();
    let mut a: Vec<i32> = getline().split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    a.sort(); a.dedup();
    println!("{}", a.len());
    for a in a {
        println!("{a}");
    }
}
