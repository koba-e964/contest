fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn yn(a: bool) -> &'static str {
    if a {
        "East"
    } else {
        "West"
    }
}

fn main() {
    let ans: i32 = getline().trim().chars().map(|c| if c == 'E' { 1 } else { -1 }).sum();
    println!("{}", yn(ans > 0));
}
