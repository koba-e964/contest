fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    getline();
    let s = getline();
    let t = getline();
    println!("{}", s.chars().zip(t.chars()).filter(|&(a, b)| a != b).count());
}
