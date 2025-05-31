fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let ab = getline().trim().split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let a = ab[0];
    let b = ab[1];
    println!("{}", (2 * a + b) / (2 * b));
}
