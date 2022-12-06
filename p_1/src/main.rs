fn main() 
{
	let limit = 1000;
	let mut sum = 0;

	for i in 1..limit
	{
		if 
		(i % 3 == 0) ||
		(i % 5 == 0)
		{
			sum += i;
		}
	}

	println!("Sum of all multiples of 3 or 5 below {}: {}", limit, sum);
}
