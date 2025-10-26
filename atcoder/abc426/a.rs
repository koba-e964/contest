fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let x = getline().trim().split_whitespace()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
    let a = ["Ocelot", "Serval", "Lynx"];
    let f = |s| {
        for i in 0..3 {
            if s == a[i] {
                return i;
            }
        }
        0
    };
    println!("{}", if f(&x[0]) >= f(&x[1]) {
        "Yes"
    } else {
        "No"
    });
}
