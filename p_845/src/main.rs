fn 
main() 
{
	let n = 10u64.pow(16);

	let mut counter = 1;
	let mut prime_checker = PrimeChecker::new();
	let mut current_positive_integer = vec![2u8];
	let mut result = current_positive_integer.clone();

	while counter <= n
	{
		let digit_sum: u64 = current_positive_integer.iter().map(|digit| *digit as u64).sum();
		if prime_checker.check(digit_sum)
		{
			if counter % (n/100) == 0
			{
				println!("{} {} {}", 
					counter, 
					current_positive_integer.clone().into_iter().rev().map(|digit| digit.to_string()).collect::<Vec<_>>().join(""), 
					digit_sum
				);
			}
			
			counter += 1;
			result = current_positive_integer.clone();

			let add_value = if current_positive_integer.first().unwrap() == &9 || digit_sum == 2 {1} else {2};

			incremet_int(&mut current_positive_integer, add_value);
		}
		else
		{
			incremet_int(&mut current_positive_integer, 1);
		}
	}

	println!("{}", result.clone().into_iter().rev().map(|digit| digit.to_string()).collect::<Vec<_>>().join(""));
}

fn
incremet_int
(
	vec: &mut Vec<u8>,
	add_value: u8
)
{
	let mut carry = add_value;
	for i in 0..vec.len()
	{
		let sum = vec.iter().nth(i).unwrap_or(&0) + carry;
		let digit = sum % 10;
		carry = (sum - digit) / 10;
		vec[i] = digit;
	}

	while carry > 0
	{
		let digit = carry % 10;
		carry = (carry - digit) / 10;
		vec.push(digit);
	}
}

struct
PrimeChecker
{
	primes: Vec<u64>
}

impl
PrimeChecker
{
	fn
	new
	()
	-> PrimeChecker
	{
		PrimeChecker { primes: vec![2, 3, 5, 7] }
	}

	fn
	check
	(
		&mut self,
		candidate: u64
	)
	-> bool
	{
		if candidate == 1
		{
			return false;
		}

		while (*self.primes.last().unwrap() as f64) < (candidate as f64).sqrt()
		{
			println!("need more primes");
			let mut next_prime = *self.primes.last().unwrap() + 2;
			while !self.check(next_prime)
			{
				next_prime += 2;
			}
			self.primes.push(next_prime);
		}

		for prime in &self.primes
		{
			if (*prime as f64) > (candidate as f64).sqrt()
			{
				return true;
			}

			if candidate % prime == 0
			{
				return false;
			}
		}

		todo!()
	}
}