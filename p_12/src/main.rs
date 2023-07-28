fn 
main() 
{
	let mut i = 1;
	let primes = create_n_primes(6);

	loop
	{
		let number = get_triangular_number(i);
		i += 1;

		let mut should_continue = false;
		for prime in &primes
		{
			if number % prime != 0
			{
				should_continue = true;
				break;
			}
		}

		if should_continue
		{
			continue;
		}
		
		let divisor_count = get_divisors(number).len();

		println!("{}: {}", number, divisor_count);

		if divisor_count > 500
		{
			break;
		}
	}
}

fn
get_triangular_number
(
	i: u64
)
-> u64
{
	(1..i).sum::<u64>()
}

fn
get_divisors
(
	number: u64
)
-> Vec<u64>
{
	let mut divisors = Vec::new();

	for candidate in 1..=number/2+1
	{
		if number % candidate == 0
		{
			divisors.push(candidate);
		}
	}

	divisors.push(number);
	return divisors;
}

fn
create_n_primes
(
	n: usize
)
-> Vec<u64>
{
	let mut primes = vec![2, 3, 5];
	let mut candidate = 7;

	while primes.len() < n
	{
		let mut is_prime = true;
		for prime in &primes
		{
			if candidate % prime == 0
			{
				is_prime = false;
				break;
			}
			
		}

		if is_prime
		{
			primes.push(candidate)
		}

		candidate += 2;
	}
	
	return primes;
}
