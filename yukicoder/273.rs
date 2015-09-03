fn getline() -> String {
  let mut ret = String::new();
  std::io::stdin().read_line(&mut ret).ok();
  return ret;
}

fn parse<T : std::str::FromStr>(s : &str) -> T {
  match s.parse::<T>() {
    Ok(t) => t,
    _    => panic!(),
  }
}

fn pal(s : &str) -> bool {
  return s.chars().rev().collect::<String>() == s;
}

fn main() {
  let str = getline().trim().to_string();
  let n = str.len();
  let mut m = 0;
  for i in 0 .. n {
    for j in i .. n {
      if i == 0 && j == n - 1 { continue; }
      if pal(&str[i .. (j + 1)]) {
        m = std::cmp::max(m, j - i + 1);
      }
    }
  }
  println!("{}", m);
}
