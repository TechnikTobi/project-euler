fn 
main() 
{
	let (a, b, c) = find_pythagorean_triplet(1000, 1000, 1000, 1000);
	println!("{} * {} * {} = {}", a, b, c, a*b*c);
}

fn
find_pythagorean_triplet
(
	sum: u64,
	max_a: u64,
	max_b: u64,
	max_c: u64,
)
-> (u64, u64, u64)
{
	for a in 1..max_a
	{
		for b in a..max_b
		{
			for c in b..max_c
			{
				if a*a + b*b != c*c
				{
					continue;
				}

				if a + b + c != sum
				{
					continue;
				}

				return (a, b, c);
			}
		}
	}

	return (0, 0, 0);
}
