fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    let mut s = getline();
    s.pop(); s.pop(); s.push('4');
    println!("{}", s);
}
