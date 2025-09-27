fn getline() -> String {
    let mut ret = String::new();
    std::io::stdin().read_line(&mut ret).unwrap();
    ret
}

fn main() {
    let n: usize = getline().trim().parse().unwrap();
    let a: Vec<i32> = getline()
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();
    let mut f = vec![0; n];
    for i in 0..n {
        if a[i] > 0 {
            f[a[i] as usize - 1] += 1;
        }
    }
    for i in 0..n {
        if f[i] > 1 {
            println!("No");
            return;
        }
    }
    let mut vac = vec![];
    for i in 0..n {
        if f[i] == 0 {
            vac.push(i as i32 + 1);
        }
    }
    let mut p = a.clone();
    for i in 0..n {
        if p[i] < 0 {
            p[i] = vac.pop().unwrap();
        }
    }
    println!("Yes");
    for i in 0..n {
        print!("{}{}", p[i], if i == n - 1 { "\n" } else { " " });
    }
}
