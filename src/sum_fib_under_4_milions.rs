fn next_fibonacci(mut a: i64, mut b: i64) -> (i64, i64) {
	let c: i64 = a + b;
	a = b;
	b = c;
	(a, b)
}

fn sum_even(max: i64) -> i64 {
	let (mut a, mut b, mut sum): (i64, i64, i64) = (0, 1, 0);
	loop {
		if b % 2 == 0 {
			sum += b;
		}

		let x = next_fibonacci(a, b);
		println!("{:?}", b);
		a = x.0;
		b = x.1;

		if b > max {
			break;
		}
	}

	println!("finished at {:?} {:?}", a,  b);
	sum
}

fn main() {
	let sum = sum_even(4000000);
	println!("sum: {:?}", sum);
}

#[cfg(test)]
mod tests {
	use *;

	#[test]
	fn test_multiples() {
		assert_eq!(sum_even(21), 10);
		assert_eq!(sum_even(56), 44);
	}
	#[test]
	fn test_fibonacci() {
		assert_eq!(next_fibonacci(1, 1), (1, 2));
		assert_eq!(next_fibonacci(34, 55), (55, 89));
	}

}
