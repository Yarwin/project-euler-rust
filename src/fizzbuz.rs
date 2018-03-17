fn fizz_buzz(n: i32) -> String {

	let mut out: String = String::new();
	if n % 3 == 0 { out.push_str("fizz"); }
	if n % 5 == 0 { out.push_str("buzz"); }
	if n % 3 != 0 && n % 5 != 0 { out.push_str(&n.to_string()); }
	out
}


fn main() {
	for i in 1..100 
	{
		println!("{:?}", fizz_buzz(i));
	}
}
