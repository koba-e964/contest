fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let s = getline();
    if s.chars().next() == Some('2') {
        println!("Success");
    } else {
        println!("Failure");
    }
}
