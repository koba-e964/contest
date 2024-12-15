fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let x: i32 = getline().trim().parse().unwrap();
    let mut s = x / 2;
    if x % 2 == 1 {
        s += 3;
    }
    println!("{s}");
}
