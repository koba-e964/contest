fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn yn(a: bool) -> &'static str {
    if a {
        "Yes"
    } else {
        "No"
    }
}

fn main() {
    let n = getline().trim().parse::<i32>().unwrap();
    println!("{}", yn(n % 111 == 0));
}
