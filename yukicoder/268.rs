fn getline() -> String {
	let mut ret = String::new();
	std::io::stdin().read_line(&mut ret).ok();
	return ret;
}

fn main() {
  let lens : Vec<i32> = getline().trim().split(" ").map(|x| x.parse().unwrap()).collect();
  let mut costs : Vec<i32> = getline().trim().split(" ").map(|x| x.parse().unwrap()).collect();
  let mut fix : Vec<i32> = vec![lens[0] + lens[1], lens[1] + lens[2], lens[2] + lens[0]];
  fix.sort();
  costs.sort();
  costs.reverse();
  println!("{}", 2 * fix.iter().zip(costs.iter()).map(|(a, b)| a * b).fold(0, |a, b| a + b));
}
