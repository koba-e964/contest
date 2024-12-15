fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    getline();
    let mut s = String::new();
    for c in getline().trim().chars() {
        s.push(match c {
            'J' => 'O',
            'O' => 'I',
            'I' => 'J',
            _ => panic!(),
        });
    }
    println!("{s}");
}
