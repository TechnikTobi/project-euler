fn 
main() 
{
    println!("Hello, world!");
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
-> Vec<u64>
{
	let mut primes = Vec::new();

	for x in (3..(val+1)).step_by(2)
	{
		primes.push(x);
	}
	todo!()
}
