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

fn main() {
  
}
