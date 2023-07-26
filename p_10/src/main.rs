fn 
main() 
{
	println!("Hello, world! {:?}", create_primes_until(100));
	println!("{}", create_primes_until(2000000).iter().sum::<u64>());
}


fn
create_primes_until
(
	limit: u64
)
-> Vec<u64>
{
	let mut primes = vec![2, 3, 5];
	let mut candidate = 7;

	while primes.last().unwrap() < &limit
	{
		let mut is_prime = true;
		for prime in &primes
		{
			if *prime as f64 > (candidate as f64).sqrt()
			{
				break;
			}

			if candidate % prime == 0
			{
				is_prime = false;
				break;
			}
			
		}

		if is_prime
		{
			println!("{}", candidate);
			primes.push(candidate)
		}

		candidate += 2;
	}
	
	// Remove last prime that exceeds the limit
	primes.pop();
	return primes;
}
