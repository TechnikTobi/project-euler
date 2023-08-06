fn 
main() 
{
	let low = 1;
	let high = 20;
	let candidate = construct_candidate(low, high);

	// println!("{}", candidate);
	println!("{}", reduce(low, high, candidate));
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
	let mut working_value = value;
	let mut processed = Vec::new();
	for i in low..=high
	{

		if i <= 1
		{
			continue;
		}

		let mut check_this = true;
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
			working_value = reduce_by(low, high, working_value, i);
			processed.push(i);
		}
	}
	return working_value;
}

fn
reduce_by
(
	low: u64,
	high: u64,
	value: u64,
	reducer: u64,
)
-> u64
{
	let mut working_value = value;
	let mut can_be_reduced = true;

	while can_be_reduced
	{
		working_value = working_value / reducer;

		for i in low..=high
		{
			if working_value % i != 0
			{
				working_value = working_value * reducer;
				can_be_reduced = false;
				break;
			}	
		}
	}

	return working_value;
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