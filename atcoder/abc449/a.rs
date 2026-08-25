fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let d = getline().trim().parse::<f64>().unwrap();
    println!("{}", d * d * std::f64::consts::PI / 4.0);
}
