fn 
main() 
{
	let primes = create_n_primes(10001);
	println!("{:?}", primes.last().unwrap());
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
