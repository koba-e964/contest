fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let n: i32 = getline().trim().parse().unwrap();
    println!("{}", if n % 11 == 0 { 1 } else { 0 });
}
