fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let ints: Vec<i32> = getline().split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let s = ints[1];
    let mut t: Vec<i32> = getline().split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    t.insert(0, 0);
    println!("{}", if t.windows(2).all(|w| w[1] - w[0] <= s) {
        "Yes"
    } else {
        "No"
    });
}
