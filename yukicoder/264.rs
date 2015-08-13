fn getline() -> String {
	let mut ret = String::new();
	std::io::stdin().read_line(&mut ret).ok();
	return ret;
}


fn main() {
  let line = getline();
  let strs : Vec<_> = line.trim().split(" ").collect();
  let n : i32 = strs[0].parse().unwrap();
  let k : i32 = strs[1].parse().unwrap();
  let msg = match (n - k + 3) % 3 {
    0 => "Drew",
    1 => "Lost",
    2 => "Won",
    _ => "",
  };
  println!("{}", msg);
}
