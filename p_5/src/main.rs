fn 
main() 
{
	primes_until(100);
}

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
// -> Vec<u64>
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
	println!("{:?}", primes);
	// todo!()
}
