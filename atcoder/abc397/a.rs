fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let x: f64 = getline().trim().parse().unwrap();
    if x < 37.5 {
        println!("3");
    } else if x < 38.0 {
        println!("2");
    } else {
        println!("1");
    }
}
