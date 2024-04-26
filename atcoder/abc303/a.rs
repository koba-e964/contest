fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).ok().unwrap();
    ret
}

fn main() {
    getline();
    let s = getline();
    let t = getline();
    let f = |x| if x == 'l' {
        '1'
    } else if x == 'o' {
        '0'
    } else {
        x
    };
    for (s, t) in s.chars().zip(t.chars()) {
        if f(s) != f(t) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
