// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?
// https://projecteuler.net/problem=3

fn main() {
	let mut x = 25;
	not_main(&x);
	println!("{:?}", x);

}


fn not_main(mut x: &i32) -> i32 {
	*x + 2
}

#[cfg(test)]
mod tests {
	use *;

	#[test]
	fn test_largest_prime() {
	}
	#[test]
	fn is_number_a_prime() {
	}
}
