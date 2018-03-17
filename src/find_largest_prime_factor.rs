// The prime factors of 13195 are 5, 7, 13 and 29.
// What is the largest prime factor of the number 600851475143 ?
// https://projecteuler.net/problem=3

fn main() {
	find_largest_prime_factor(600851475143);
}

fn find_largest_prime_factor(number: i64) -> i64 {
let mut largest_prime: i64 = 1;

    for i in 1.. ((number as f64).sqrt() as i64 + 1) {
        if is_prime(i) && (number % i == 0) {
            largest_prime = i;
        }
    }

	println!("{} is the largest prime factor of {}.", largest_prime, number);
    largest_prime
}

fn is_prime(number: i64) -> bool {
    match number {
        1 => return false,
        _ => {
            for i in 2..(((number as f64).sqrt() as i64) + 1) {
                if number % i == 0 {
                    return false;
                }
            }
            return true;
        }
    }
}

#[cfg(test)]
mod tests {
	use *;

	#[test]
	fn test_largest_prime() {
		assert_eq!(find_largest_prime_factor(13195), 29);
	}
	#[test]
	fn is_number_a_prime() {
		assert_eq!(is_prime(1), false);
		assert_eq!(is_prime(29), true);
	}
}
