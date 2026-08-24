fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let x = getline().trim().parse::<i32>().unwrap();
    println!("{}", if (3..=18).contains(&x) { "Yes" } else { "No" });
}
