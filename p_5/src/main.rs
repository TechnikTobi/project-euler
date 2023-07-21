fn 
main() 
{
	// primes_until(100);
	let low = 1;
	let high = 20;
	let candidate = construct_candidate(low, high);

	println!("{}", candidate);

	
}

fn
construct_candidate
(
	low: u64,
	high: u64
)
-> u64
{
	let mut result = 1;
	for i in low..=high
	{
		result *= i;
	}	
	return result;
}

fn
reduce
(
	low: u64,
	high: u64,
	value: u64,
)
-> u64
{
	let out working_value = value;
	let mut processed = Vet::new();
	for i in low..=high
	{
		let out check_this = true;
		for j in &processed 
		{
			if i % j == 0
			{
				check_this = false;
				break;
			}
		}
		if check_this
		{
			
		}
	}
}

/*
/// Computes the prime decomposition of val
fn
prime_decomp
(
	val: u64
)
-> Vec<u64>
{
	
	todo!()
}

/// Computes primes smaller than val
fn
primes_until
(
	val: u64
)
-> Vec<u64>
{
	let mut primes = Vec::new();

	for x in (3..(val+1)).step_by(2)
	{
		let mut is_prime = true;
		for prime in &primes
		{
			if x % prime == 0
			{
				is_prime = false;
				break;
			}
		}

		if is_prime
		{
			primes.push(x);
		}
	}

	return primes;
	// println!("{:?}", primes);
	// todo!()
}
*/