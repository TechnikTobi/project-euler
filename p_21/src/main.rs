use std::collections::HashSet;

fn 
main() 
{
	let mut amicable_numbers = HashSet::new();
	for number in 1..10000
	{
		if amicable_numbers.contains(&number)
		{
			continue;
		}

		if let Some(partner) = has_amicable_partner(number)
		{
			amicable_numbers.insert(number);
			amicable_numbers.insert(partner);
		}
	}

	println!("{:?}", amicable_numbers);
	println!("{}", amicable_numbers.iter().sum::<u64>());
}

fn
has_amicable_partner
(
	number: u64
)
-> Option<u64>
{
	let divisors = get_divisors(number);
	let partner = divisors.iter().sum::<u64>();

	if partner == number
	{
		return None;
	}

	let partner_divisors = get_divisors(partner);
	if partner_divisors.iter().sum::<u64>() == number
	{
		return Some(partner);
	}

	return None;
}

fn
get_divisors
(
	number: u64
)
-> Vec<u64>
{
	let mut divisors = vec![1];
	for divisor in 2..=(number/2+1)
	{
		if number % divisor == 0
		{
			divisors.push(divisor);
		}
	}

	return divisors;
}