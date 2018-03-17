
// https://projecteuler.net/problem=1
// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
// Find the sum of all the multiples of 3 or 5 below 1000.


fn return_n_if_multiple(a: i32, b:i32, n: i32) ->  i32 {
	match n {
		n if (n % a == 0 || n % b == 0) => n,
		_ => 0
	}
}

fn sum_multiples(a: i32, b:i32, n: i32) -> i32 {
	let mut sum: i32 = 0;
	for i in 1..n {
		sum += return_n_if_multiple(a, b, i);
	}
	sum
}


fn main() {
	println!("{:?}", sum_multiples(3, 5, 1000));	
}


#[cfg(test)]
mod tests {
	use *;

    #[test]
    fn test_multiples() {
    	assert_eq!(return_n_if_multiple(3, 5, 5), 5);
    	assert_eq!(return_n_if_multiple(3, 5, 3), 3);
    	assert_eq!(return_n_if_multiple(3, 5, 15), 15);
    }
    #[test]
    fn test_example() {
    	assert_eq!(sum_multiples(3, 5, 10), 23);
    }

}
