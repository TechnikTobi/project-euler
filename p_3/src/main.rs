fn find_next_prime
(
	primes: &mut Vec<u64>
)
{
	let current_largest_prime = primes[primes.len()-1];
	let mut is_prime = false;
	let mut current = current_largest_prime+1;
	while !is_prime
	{
		let root = (current as f64).sqrt() as u64 + 1;
		let mut index = 0;
		is_prime = true;
		while primes[index] <= root
		{
			if current % primes[index] == 0
			{
				is_prime = false;
				current += 1;
				break;
			}

			index += 1;

			if index == primes.len()
			{
				break;
			}
		}	
	}

	primes.push(current);
}


fn main()
{
	let mut number = 600851475143u64;
	let mut primes = vec!(2);
	
	while number > 1
	{
		let mut factor_found = false;
		let mut current_factor_index = 0;

		while !factor_found
		{
			if current_factor_index == primes.len()
			{
				find_next_prime(&mut primes);
			}

			if number % primes[current_factor_index] == 0
			{
				factor_found = true;
				number = number / primes[current_factor_index];
				println!("{} {}", primes[current_factor_index], number);
			}

			current_factor_index += 1;
		} 
	}
}

/*
fn create_primes
(
	limit: u64
)
-> Vec<u64>
{
	let mut primes = vec!(2u64);
	for candidate in 3..limit
	{
		let mut index = 0;
		let root = (candidate as f64).sqrt() as u64 + 1;
		let mut is_prime = true;
		while primes[index] <= root
		{
			if candidate % primes[index] == 0
			{
				is_prime = false;
				break;
			}

			index += 1;

			if index == primes.len()
			{
				break;
			}
		}

		if is_prime
		{
			println!("{}", candidate);
			primes.push(candidate);
		}
	}

	return primes;
}



fn main() 
{
	let number = 600851475143;
	let mut primes = create_primes(number);
	primes.reverse();

	for prime in primes
	{
		if number % prime == 0
		{
			println!("Largest prime factor of {}: {}", number, prime);
			return;
		}
	}
}
*/
